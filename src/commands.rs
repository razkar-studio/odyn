use std::path::PathBuf;

use crate::{
    constants::{
        APACHE_LICENSE, BSD2_LICENSE, BSD3_LICENSE, DEPS_DIR, GPL3_LICENSE, ISC_LICENSE,
        MIT_LICENSE, MPL2_LICENSE, OLS_JSON, UNLICENSE, VERSION, ZLIB_LICENSE,
    },
    storage::{
        Dep, DepState, Lockfile, check_git, gen_main_odin, load_lockfile, save_lockfile,
        save_lockfile_at,
    },
    ui::status,
};
use anyhow::{Result, anyhow};
use farben::ceprintln;
use sha2::{Digest, Sha256};

pub(crate) fn cmd_init(
    project_name: String,
    license: String,
    with_readme: bool,
    no_src: bool,
) -> Result<()> {
    let root: PathBuf = PathBuf::from(&project_name);
    if root.exists() {
        return Err(anyhow!("directory '{project_name}' already exists"));
    }

    std::fs::create_dir_all(root.join(DEPS_DIR))?;
    if no_src {
        std::fs::write(root.join("main.odin"), gen_main_odin(&project_name))?;
    } else {
        std::fs::create_dir_all(root.join("src"))?;
        std::fs::write(
            root.join("src").join("main.odin"),
            gen_main_odin(&project_name),
        )?;
    }

    if with_readme {
        std::fs::write(root.join("README.md"), format!("# {project_name}\n"))?;
    }

    std::fs::write(root.join("ols.json"), OLS_JSON)?;

    save_lockfile_at(&Lockfile { dep: Vec::new() }, &root)?;
    let license_content: String = match license.to_lowercase().as_str() {
        "mit" => MIT_LICENSE.to_string(),
        "apache" => APACHE_LICENSE.to_string(),
        "gpl3" => GPL3_LICENSE.to_string(),
        "bsd2" => BSD2_LICENSE.to_string(),
        "bsd3" => BSD3_LICENSE.to_string(),
        "mpl2" => MPL2_LICENSE.to_string(),
        "unlicense" => UNLICENSE.to_string(),
        "zlib" => ZLIB_LICENSE.to_string(),
        "isc" => ISC_LICENSE.to_string(),
        other => format!("License: {other}\n"),
    };

    std::fs::write(root.join("LICENSE"), license_content)?;
    Ok(())
}

pub(crate) fn cmd_update_self() -> Result<()> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let binary_name = match (os, arch) {
        ("linux", "x86_64") => "odyn-linux-x86_64",
        ("linux", "aarch64") => "odyn-linux-aarch64",
        ("linux", "x86") => "odyn-linux-i686",
        ("linux", "riscv64") => "odyn-linux-riscv64",
        ("linux", "arm") => "odyn-linux-armv7", // defaults to armv7, armv6 users install manually
        ("linux", "powerpc64") => "odyn-linux-powerpc64le", // defaults to LE, modern POWER
        ("linux", "s390x") => "odyn-linux-s390x",
        ("linux", "sparc64") => "odyn-linux-sparc64",
        // MIPS: all variants failed, build from source
        // musl: can't detect at runtime, install manually from Releases
        ("windows", "x86_64") => "odyn-windows-x86_64.exe",
        ("windows", "x86") => "odyn-windows-i686.exe",

        ("macos", "x86_64") => "odyn-macos-x86_64",
        ("macos", "aarch64") => "odyn-macos-aarch64",

        ("android", "x86_64") => "odyn-android-x86_64",
        ("android", "aarch64") => "odyn-android-aarch64",
        ("android", "arm") => "odyn-android-armv7",

        ("freebsd", "x86_64") => "odyn-freebsd-x86_64",
        ("freebsd", "x86") => "odyn-freebsd-i686",

        ("netbsd", "x86_64") => "odyn-netbsd-x86_64",
        _ => {
            return Err(anyhow!(
                "unsupported platform ({os}/{arch}). install manually from https://codeberg.org/razkar/odyn/releases, use Cargo, or build from source."
            ));
        }
    };

    let body = ureq::get("https://codeberg.org/api/v1/repos/razkar/odyn/releases/latest")
        .call()
        .map_err(|e| anyhow!("failed to fetch latest release: {e}"))?
        .body_mut()
        .read_to_string()?;

    let latest = body
        .split("\"tag_name\":\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .ok_or_else(|| anyhow!("could not parse release info from Codeberg API"))?
        .trim_start_matches('v');

    if latest == VERSION {
        status(
            "UpToDate",
            "success",
            &format!("already on latest version ({VERSION})"),
        );
        return Ok(());
    }

    status(
        "Update",
        "load",
        &format!("new version available: {VERSION} → {latest}"),
    );

    let url = format!("https://codeberg.org/razkar/odyn/releases/download/v{latest}/{binary_name}");
    let temp_path = std::env::temp_dir().join(binary_name);

    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow!("failed to download update: {e}"))?;

    let mut temp_file = std::fs::File::create(&temp_path)?;
    std::io::copy(&mut response.into_body().as_reader(), &mut temp_file)?;

    let metadata = std::fs::metadata(&temp_path)?;
    if metadata.len() == 0 {
        std::fs::remove_file(&temp_path)?;
        return Err(anyhow!("downloaded file is empty, aborting update"));
    }

    let sums_filename = if os == "macos" {
        "SHA256SUMS-macos"
    } else {
        "SHA256SUMS"
    };

    let sums = ureq::get(&format!(
        "https://codeberg.org/razkar/odyn/releases/download/v{latest}/{sums_filename}"
    ))
    .call()
    .map_err(|e| anyhow!("failed to fetch SHA256SUMS: {e}"))?
    .body_mut()
    .read_to_string()?;

    let expected = sums
        .lines()
        .find(|l| l.contains(binary_name))
        .and_then(|l| l.split_whitespace().next())
        .ok_or_else(|| anyhow!("could not find hash for '{binary_name}' in SHA256SUMS"))?;

    let bytes = std::fs::read(&temp_path)?;
    let hash = Sha256::digest(&bytes);
    let actual = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    if actual != expected {
        std::fs::remove_file(&temp_path)?;
        return Err(anyhow!(
            "SHA256 mismatch! expected {expected}, got {actual}. aborting update."
        ));
    }
    status("Verified", "success", "SHA256 checksum verified");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o755))?;
    }

    let current_exe = std::env::current_exe()?;

    #[cfg(target_os = "windows")]
    std::fs::rename(&current_exe, current_exe.with_extension("exe.old"))?;

    std::fs::rename(&temp_path, &current_exe)?;

    status("Updated", "success", &format!("odyn updated to v{latest}"));
    status("Info", "info", "restart odyn for changes to take effect");

    #[cfg(target_os = "windows")]
    std::fs::remove_file(current_exe.with_extension("exe.old")).ok();

    Ok(())
}

pub(crate) fn cmd_remove(name: String) -> Result<()> {
    let mut lockfile: Lockfile = load_lockfile()?;
    if !lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow!("dependency '{name}' not found in Odyn.lock"));
    }
    std::fs::remove_dir_all(PathBuf::from(DEPS_DIR).join(&name))?;
    lockfile.dep.retain(|d| d.name != name);
    save_lockfile(&lockfile)?;
    Ok(())
}

pub(crate) fn cmd_update(name: String) -> Result<()> {
    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;
    if !lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow!("dep '{name}' does not exist"));
    }
    let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&name);

    status("Updating", "load", &format!("'{name}'"));

    let fetch_status = std::process::Command::new("git")
        .args(["fetch", "origin", "--quiet"])
        .current_dir(&dep_path)
        .status()?;
    if !fetch_status.success() {
        return Err(anyhow!("failed to fetch '{name}'"));
    }

    let reset_status = std::process::Command::new("git")
        .args(["reset", "--hard", "--quiet", "origin/HEAD"])
        .current_dir(&dep_path)
        .status()?;
    if !reset_status.success() {
        return Err(anyhow!("failed to reset '{name}' to origin/HEAD"));
    }

    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&dep_path)
        .output()?;
    let commit = String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
        .trim()
        .to_string();

    if let Some(dep) = lockfile.dep.iter_mut().find(|d| d.name == name) {
        dep.commit = commit.clone();
    }

    save_lockfile(&lockfile)?;
    status(
        "Updated",
        "success",
        &format!("'{name}' → {}", &commit[..7]),
    );
    Ok(())
}

pub(crate) fn cmd_status() -> Result<()> {
    check_git()?;
    let lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.is_empty() {
        status("Status", "info", "no dependencies");
        return Ok(());
    }

    let mut any_bad = false;

    for dep in &lockfile.dep {
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);

        if !dep_path.exists() {
            status("Missing", "error", &format!("'{}'", dep.name));
            any_bad = true;
            continue;
        }

        let output = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&dep_path)
            .output()?;
        let commit = String::from_utf8(output.stdout)
            .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
            .trim()
            .to_string();

        if commit == dep.commit {
            status(
                "Ok",
                "success",
                &format!("'{}' at {}", dep.name, &dep.commit[..7]),
            );
        } else {
            status(
                "Modified",
                "error",
                &format!(
                    "'{}': expected '{}' but found '{}'",
                    dep.name,
                    &dep.commit[..7],
                    &commit[..7]
                ),
            );
            any_bad = true;
        }
    }

    if any_bad {
        return Err(anyhow!("some dependencies are missing or modified"));
    }

    Ok(())
}

pub(crate) fn cmd_sync(force: bool, skip: Vec<String>) -> Result<()> {
    check_git()?;
    let lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.is_empty() {
        status("Sync", "info", "no dependencies to sync");
        return Ok(());
    }

    let mut result: Vec<(&Dep, DepState)> = Vec::new();
    for dep in &lockfile.dep {
        if skip.contains(&dep.name) {
            continue;
        }
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        if dep_path.exists() {
            let output = std::process::Command::new("git")
                .arg("rev-parse")
                .arg("HEAD")
                .current_dir(dep_path)
                .output()?;
            let commit = String::from_utf8(output.stdout)
                .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
                .trim()
                .to_string();
            if commit == dep.commit {
                result.push((dep, DepState::Ok))
            } else {
                if force {
                    result.push((dep, DepState::Missing))
                } else {
                    result.push((dep, DepState::Modified { actual: commit }))
                }
            }
        } else {
            result.push((dep, DepState::Missing));
        }
    }

    let modified: Vec<_> = result
        .iter()
        .filter_map(|(dep, actual)| {
            if let DepState::Modified { actual } = actual {
                Some((*dep, actual))
            } else {
                None
            }
        })
        .collect();

    if !modified.is_empty() {
        ceprintln!("[error]       Error[/] some deps have local changes:");
        for (dep, actual) in modified {
            ceprintln!(
                "[error]       Error[/] '{}': expected '{}' but found '{}'",
                dep.name,
                &dep.commit[..7],
                &actual[..7]
            );
        }
        status("Hint", "info", "revert local changes to run sync");
        return Err(anyhow!("sync failed: modified dependencies found"));
    }

    for (dep, state) in &result {
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        match state {
            DepState::Missing => {
                if !dep_path.exists() {
                    status("Syncing", "load", &format!("'{}', cloning...", dep.name));
                    let clone_status = std::process::Command::new("git")
                        .arg("clone")
                        .arg(&dep.source)
                        .arg(&dep_path)
                        .status()?;
                    if !clone_status.success() {
                        return Err(anyhow!("failed to clone '{}'", dep.name));
                    }
                } else {
                    status(
                        "Resetting",
                        "load",
                        &format!("'{}' to pinned commit...", dep.name),
                    );
                }
                let reset_status = std::process::Command::new("git")
                    .args(["reset", "--hard", dep.commit.as_str()])
                    .current_dir(&dep_path)
                    .status()?;
                if !reset_status.success() {
                    return Err(anyhow!("failed to reset '{}' to pinned commit", dep.name));
                }
                status("Synced", "done", &format!("'{}'", dep.name));
            }
            DepState::Ok => {
                status("Verified", "info", &format!("'{}'", dep.name));
            }
            _ => {}
        }
    }

    status("Finished", "success", "all dependencies up to date");
    Ok(())
}

pub(crate) fn cmd_get(
    source: String,
    name: Option<String>,
    platform: String,
    commit: Option<String>,
) -> Result<()> {
    let looks_local = source.starts_with('/')
        || source.starts_with("./")
        || source.starts_with("../")
        || source.starts_with('~')
        || source.starts_with(".\\")
        || source.starts_with("..\\")
        || source.starts_with("file://")
        || source.len() >= 3
            && source
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic())
            && source.chars().nth(1) == Some(':')
            && (source.chars().nth(2) == Some('\\') || source.chars().nth(2) == Some('/'));

    if looks_local {
        return Err(anyhow!(
            "local paths aren't supported. push '{source}' to a Git remote and use that URL instead"
        ));
    }

    let source = if !source.contains("://") && source.matches('/').count() == 1 {
        let base = match platform.to_lowercase().as_str() {
            "github" => "https://github.com",
            "codeberg" => "https://codeberg.org",
            "gitlab" => "https://gitlab.com",
            "sourcehut" | "sr.ht" => "https://git.sr.ht",
            "bitbucket" => "https://bitbucket.org",
            "gitea" => {
                return Err(anyhow!(
                    "'gitea' has no single public instance, use a full URL instead, e.g. https://your-gitea.com/{{source}}"
                ));
            }
            "savannah" => "https://git.savannah.gnu.org/git",
            "notabug" => "https://notabug.org",
            "disroot" => "https://git.disroot.org",
            "framagit" => "https://framagit.org",
            other => {
                return Err(anyhow!(
                    "unknown platform '{other}'. use a full URL instead, e.g. https://{other}/{source}"
                ));
            }
        };
        format!("{base}/{source}")
    } else {
        source
    };

    let name: String = name.unwrap_or_else(|| {
        source
            .split('/')
            .next_back()
            .unwrap_or("unknown")
            .to_string()
    });
    let name: String = name.strip_suffix(".git").unwrap_or(&name).to_string();

    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.iter().any(|dep| dep.source == source) {
        status("Skipping", "warn", &format!("'{name}' already exists"));
        return Ok(());
    }

    let dep_path = PathBuf::from(DEPS_DIR).join(&name);

    std::fs::create_dir_all(PathBuf::from(DEPS_DIR))?;
    let exit_status = std::process::Command::new("git")
        .arg("clone")
        .arg(&source)
        .arg(&dep_path)
        .status()?;

    if !exit_status.success() {
        return Err(anyhow!("git clone failed"));
    }

    let commit = match commit {
        Some(c) => {
            let checkout_status = std::process::Command::new("git")
                .args(["checkout", c.as_str()])
                .current_dir(&dep_path)
                .status()?;
            if !checkout_status.success() {
                return Err(anyhow!(
                    "failed to checkout commit '{c}'. maybe it was a typo?"
                ));
            }
            c
        }
        None => {
            let output = std::process::Command::new("git")
                .arg("rev-parse")
                .arg("HEAD")
                .current_dir(&dep_path)
                .output()?;
            String::from_utf8(output.stdout)
                .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
                .trim()
                .to_string()
        }
    };

    lockfile.dep.push(Dep {
        name: name.clone(),
        source,
        commit,
    });

    save_lockfile(&lockfile)?;
    Ok(())
}
