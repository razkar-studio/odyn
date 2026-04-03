use std::path::PathBuf;

use crate::{
    constants::{
        APACHE_LICENSE, BSD2_LICENSE, BSD3_LICENSE, DEPS_DIR, GPL3_LICENSE, ISC_LICENSE, LOCKFILE,
        MIT_LICENSE, MPL2_LICENSE, OLS_JSON, UNLICENSE, VERSION, ZLIB_LICENSE,
    },
    storage::{
        Dep, DepState, Lockfile, check_git, gen_main_odin, load_lockfile, save_lockfile,
        save_lockfile_at,
    },
    ui::status,
};
use anyhow::{Result, anyhow};
use farben::{ceprintln, cprintln};
use sha2::{Digest, Sha256};

fn short(commit: &str) -> &str {
    let end = commit
        .char_indices()
        .nth(7)
        .map(|(i, _)| i)
        .unwrap_or(commit.len());
    &commit[..end]
}

pub(crate) fn parse_version(s: &str) -> (u32, u32, u32) {
    let s = s.trim_start_matches('v');
    let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
    match parts.as_slice() {
        [major, minor, patch, ..] => (*major, *minor, *patch),
        [major, minor] => (*major, *minor, 0),
        [major] => (*major, 0, 0),
        _ => (0, 0, 0),
    }
}

pub(crate) fn version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let (a_major, a_minor, a_patch) = parse_version(a);
    let (b_major, b_minor, b_patch) = parse_version(b);
    (a_major, a_minor, a_patch).cmp(&(b_major, b_minor, b_patch))
}

fn git_head(dep_path: &PathBuf) -> Result<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dep_path)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "git rev-parse HEAD failed in '{}'",
            dep_path.display()
        ));
    }
    Ok(String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
        .trim()
        .to_string())
}

pub(crate) fn cmd_version(verbose: bool) {
    let extra = match env!("ODYN_INSTALL_METHOD") {
        "cargo" => "[ansi(173)]Cargo Edition".to_string(),
        "source" => {
            let res = "[ansi(62)]Nightly".to_string();
            match option_env!("ODYN_GIT_HASH") {
                None => res,
                Some(hash) => {
                    let mut res = res;
                    res.push_str(&format!("[/ansi(62)], commit {hash}"));
                    res
                }
            }
        }
        "release" => {
            let mut res = String::from("");
            res.push_str(match std::env::consts::OS {
                "linux" => "[yellow]Linux[/yellow]",
                "windows" => "[blue]Windows[/blue]",
                "macos" => "[ansi(250)]macOS[/ansi(250)]",
                "android" => "[green]Android[/green]",
                "freebsd" => "[bright-red]FreeBSD[/bright-red]",
                "netbsd" => "[ansi(214)]NetBSD[/ansi(214)]",
                other => other,
            });
            res.push(' ');
            res.push_str(std::env::consts::ARCH);
            res
        }
        _ => "".to_string(),
    };
    cprintln!("[bold blue]Odyn[/blue] v{VERSION} {extra}");
    println!("    Reproducible vendoring tool for the Odin programming language.");

    let git_version = std::process::Command::new("git")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    match git_version {
        Some(v) => cprintln!("    [ansi(214)]{}[/ansi(214)][/bold]", v),
        None => cprintln!("    [bright-red]Git Not Installed[/bright-red]"),
    }

    if verbose {
        let install_path = std::env::current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        cprintln!("    [dim]installed at {}[/dim]", install_path);
        cprintln!("    [dim]built on {}[/dim]", env!("ODYN_BUILD_DATE"));
    }
}

pub(crate) fn cmd_init(
    project_name: String,
    license: String,
    with_readme: bool,
    no_src: bool,
    migrate: bool,
) -> Result<()> {
    if migrate {
        let deps_dir = PathBuf::from(DEPS_DIR);
        let ols = PathBuf::from("ols.json");
        let lockfile = PathBuf::from(LOCKFILE);

        if deps_dir.exists() {
            return Err(anyhow!("'{}' already exists", DEPS_DIR));
        }
        if lockfile.exists() {
            return Err(anyhow!("'Odyn.lock' already exists"));
        }

        std::fs::create_dir_all(&deps_dir)?;

        if ols.exists() {
            let content = std::fs::read_to_string(&ols)?;
            let mut json: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| anyhow!("failed to parse ols.json: {e}"))?;
            let collections = json
                .get_mut("collections")
                .and_then(|v| v.as_array_mut())
                .ok_or_else(|| anyhow!("ols.json has no 'collections' array"))?;
            if collections
                .iter()
                .any(|e| e.get("name").and_then(|n| n.as_str()) == Some("deps"))
            {
                return Err(anyhow!("ols.json already has a 'deps' collection"));
            }
            collections.push(serde_json::json!({ "name": "deps", "path": "odyn_deps" }));
            std::fs::write(&ols, serde_json::to_string_pretty(&json)?)?;
        } else {
            std::fs::write(&ols, OLS_JSON)?;
        }

        save_lockfile_at(&Lockfile { dep: Vec::new() }, &PathBuf::from("."))?;

        return Ok(());
    }

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

pub(crate) fn cmd_update_self(pre_release: bool, nightly: bool) -> Result<()> {
    if pre_release && nightly {
        return Err(anyhow!(
            "--pre-release and --nightly cannot be used together"
        ));
    }

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    if nightly {
        let commit = ureq::get("https://codeberg.org/api/v1/repos/razkar/odyn/branches/main")
            .call()
            .ok()
            .and_then(|mut r| r.body_mut().read_to_string().ok())
            .and_then(|body| {
                body.split("\"sha\":")
                    .nth(1)?
                    .trim_start_matches([' ', '\t'])
                    .strip_prefix('"')?
                    .split('"')
                    .next()
                    .map(|s| s[..8].to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        status(
            "Nightly",
            "load",
            &format!("building from commit {commit}..."),
        );

        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let exit = std::process::Command::new(&cargo)
            .args([
                "install",
                "--git",
                "https://codeberg.org/razkar/odyn.git",
                "--force",
                "--no-default-features",
            ])
            .env("ODYN_INSTALL_METHOD", "source")
            .status()
            .map_err(|e| anyhow!("failed to run cargo: {e}"))?;

        if !exit.success() {
            return Err(anyhow!("cargo install failed"));
        }

        status(
            "Updated",
            "success",
            &format!("odyn nightly installed (commit {commit})"),
        );
        return Ok(());
    }

    let binary_name = match (os, arch) {
        ("linux", "x86_64") => "odyn-linux-x86_64",
        ("linux", "aarch64") => "odyn-linux-aarch64",
        ("linux", "x86") => "odyn-linux-i686",
        ("linux", "riscv64") => "odyn-linux-riscv64",
        ("linux", "arm") => "odyn-linux-armv7",
        ("linux", "powerpc64") => "odyn-linux-powerpc64le",
        ("linux", "s390x") => "odyn-linux-s390x",
        ("linux", "sparc64") => "odyn-linux-sparc64",
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

    let latest: String = if pre_release {
        let body = ureq::get(
            "https://codeberg.org/api/v1/repos/razkar/odyn/releases?limit=20&draft=false",
        )
        .call()
        .map_err(|e| anyhow!("failed to fetch releases: {e}"))?
        .body_mut()
        .read_to_string()?;

        body.split("\"tag_name\":")
            .skip(1)
            .find_map(|s| {
                Some(
                    s.trim_start_matches([' ', '\t'])
                        .strip_prefix('"')?
                        .split('"')
                        .next()?
                        .to_string(),
                )
            })
            .ok_or_else(|| anyhow!("no pre-release found on Codeberg"))?
            .trim_start_matches('v')
            .to_string()
    } else {
        let body = ureq::get("https://codeberg.org/api/v1/repos/razkar/odyn/releases/latest")
            .call()
            .map_err(|e| anyhow!("failed to fetch latest release: {e}"))?
            .body_mut()
            .read_to_string()?;

        body.split("\"tag_name\":")
            .nth(1)
            .and_then(|s| s.trim_start_matches([' ', '\t']).strip_prefix('"'))
            .and_then(|s| s.split('"').next())
            .ok_or_else(|| anyhow!("could not parse release info from Codeberg API"))?
            .trim_start_matches('v')
            .to_string()
    };
    let latest = latest.as_str();

    if latest == VERSION {
        status(
            "UpToDate",
            "success",
            &format!("already on latest version ({VERSION})"),
        );
        return Ok(());
    }

    if !pre_release && !nightly && version_cmp(VERSION, latest).is_gt() {
        status(
            "Newer",
            "warn",
            &format!(
                "local version ({VERSION}) is newer than latest release ({latest}). skipping."
            ),
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
    drop(temp_file);

    let metadata = std::fs::metadata(&temp_path)?;
    if metadata.len() == 0 {
        std::fs::remove_file(&temp_path).ok();
        return Err(anyhow!("downloaded file is empty, aborting update"));
    }

    let sums_filename = match os {
        "linux" | "windows" => "SHA256SUMS",
        _ => "SHA256SUMS-github",
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
        std::fs::remove_file(&temp_path).ok();
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
    {
        let old_path = current_exe.with_extension("exe.old");
        if let Err(e) = std::fs::rename(&current_exe, &old_path) {
            std::fs::remove_file(&temp_path).ok();
            return Err(anyhow!("failed to rename current binary: {e}"));
        }
        if let Err(e) = std::fs::copy(&temp_path, &current_exe) {
            std::fs::rename(&old_path, &current_exe).ok();
            std::fs::remove_file(&temp_path).ok();
            return Err(anyhow!("failed to install new binary: {e}"));
        }
        std::fs::remove_file(&temp_path).ok();
        std::fs::remove_file(&old_path).ok();
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Err(e) = std::fs::remove_file(&current_exe) {
            std::fs::remove_file(&temp_path).ok();
            return Err(anyhow!("failed to remove current binary: {e}"));
        }
        if let Err(e) = std::fs::copy(&temp_path, &current_exe) {
            std::fs::remove_file(&temp_path).ok();
            return Err(anyhow!("failed to install new binary: {e}"));
        }
        std::fs::remove_file(&temp_path).ok();
    }

    status("Updated", "success", &format!("odyn updated to v{latest}"));
    status("Info", "info", "restart odyn for changes to take effect");

    Ok(())
}

pub(crate) fn cmd_remove(name: String) -> Result<()> {
    let mut lockfile: Lockfile = load_lockfile()?;
    if !lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow!("dependency '{name}' not found in Odyn.lock"));
    }

    let dep_path = PathBuf::from(DEPS_DIR).join(&name);
    if dep_path.exists() {
        std::fs::remove_dir_all(&dep_path)?;
    }

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
        .args(["reset", "--hard", "--quiet", "FETCH_HEAD"])
        .current_dir(&dep_path)
        .status()?;
    if !reset_status.success() {
        return Err(anyhow!("failed to reset '{name}' to latest commit"));
    }

    let commit = git_head(&dep_path)?;

    if let Some(dep) = lockfile.dep.iter_mut().find(|d| d.name == name) {
        dep.commit = commit.clone();
    }

    save_lockfile(&lockfile)?;
    status(
        "Updated",
        "success",
        &format!("'{name}' → {}", short(&commit)),
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

        let commit = match git_head(&dep_path) {
            Ok(c) => c,
            Err(e) => {
                status("Error", "error", &format!("'{}': {e}", dep.name));
                any_bad = true;
                continue;
            }
        };

        if commit == dep.commit {
            status(
                "Ok",
                "success",
                &format!("'{}' at {}", dep.name, short(&dep.commit)),
            );
        } else {
            status(
                "Modified",
                "error",
                &format!(
                    "'{}': expected '{}' but found '{}'",
                    dep.name,
                    short(&dep.commit),
                    short(&commit),
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
    let mut any_skipped = false;

    for dep in &lockfile.dep {
        if skip.contains(&dep.name) {
            any_skipped = true;
            continue;
        }
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        if dep_path.exists() {
            let commit = match git_head(&dep_path) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow!("failed to read commit for '{}': {e}", dep.name));
                }
            };
            if commit == dep.commit {
                result.push((dep, DepState::Ok))
            } else if force {
                result.push((dep, DepState::Missing))
            } else {
                result.push((dep, DepState::Modified { actual: commit }))
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
                short(&dep.commit),
                short(actual)
            );
        }
        status("Hint", "info", "revert local changes or use --force");
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
                        "Fetching",
                        "load",
                        &format!("'{}' before reset...", dep.name),
                    );
                    let fetch_status = std::process::Command::new("git")
                        .args(["fetch", "origin", "--quiet"])
                        .current_dir(&dep_path)
                        .status()?;
                    if !fetch_status.success() {
                        return Err(anyhow!("failed to fetch '{}' before reset", dep.name));
                    }
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

    if any_skipped {
        status(
            "Finished",
            "success",
            "sync complete (some dependencies were skipped)",
        );
    } else {
        status("Finished", "success", "all dependencies up to date");
    }

    Ok(())
}

pub(crate) fn cmd_get(
    source: String,
    name: Option<String>,
    platform: String,
    commit: Option<String>,
    depth: Option<u32>,
    extra_args: Vec<String>,
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

    let raw_name = name.unwrap_or_else(|| {
        source
            .trim_end_matches('/')
            .split('/')
            .next_back()
            .unwrap_or("unknown")
            .to_string()
    });

    if raw_name.is_empty() {
        return Err(anyhow!(
            "could not derive a name from '{source}'. pass one explicitly: odyn get <source> <name>"
        ));
    }

    let name: String = raw_name
        .strip_suffix(".git")
        .unwrap_or(&raw_name)
        .to_string();

    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.iter().any(|dep| dep.source == source) {
        status("Skipping", "warn", &format!("'{name}' already exists"));
        return Ok(());
    }

    if lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow!(
            "a dependency named '{name}' already exists in Odyn.lock, try using a custom name and prefix the author"
        ));
    }

    let dep_path = PathBuf::from(DEPS_DIR).join(&name);

    std::fs::create_dir_all(PathBuf::from(DEPS_DIR))?;

    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");
    if let Some(n) = depth {
        cmd.args(["--depth", &n.to_string()]);
    }
    cmd.args(extra_args);
    cmd.arg(&source).arg(&dep_path);
    let exit_status = cmd.status()?;

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
                std::fs::remove_dir_all(&dep_path).ok();
                return Err(anyhow!(
                    "failed to checkout commit '{c}'. maybe it was a typo? (partial clone removed)"
                ));
            }
            match git_head(&dep_path) {
                Ok(c) => c,
                Err(e) => {
                    std::fs::remove_dir_all(&dep_path).ok();
                    return Err(anyhow!("failed to resolve HEAD after checkout: {e}"));
                }
            }
        }
        None => match git_head(&dep_path) {
            Ok(c) => c,
            Err(e) => {
                std::fs::remove_dir_all(&dep_path).ok();
                return Err(anyhow!("failed to read HEAD commit: {e}"));
            }
        },
    };

    lockfile.dep.push(Dep {
        name: name.clone(),
        source,
        commit,
    });

    if let Err(e) = save_lockfile(&lockfile) {
        std::fs::remove_dir_all(&dep_path).ok();
        return Err(anyhow!(
            "failed to save lockfile (partial clone removed): {e}"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("1.0.0"), (1, 0, 0));
        assert_eq!(parse_version("v1.0.0"), (1, 0, 0));
        assert_eq!(parse_version("1.2.3"), (1, 2, 3));
        assert_eq!(parse_version("v2"), (2, 0, 0));
        assert_eq!(parse_version("3"), (3, 0, 0));
        assert_eq!(parse_version(""), (0, 0, 0));
        assert_eq!(parse_version("abc"), (0, 0, 0));
        assert_eq!(parse_version("1.2.3.4"), (1, 2, 3));
    }

    #[test]
    fn test_version_cmp() {
        assert_eq!(version_cmp("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
        assert_eq!(version_cmp("1.0.0", "2.0.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("2.0.0", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("1.0.0", "1.1.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.1.0", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("1.0.1", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_cmp("0.3.0", "0.10.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("v1.0.0", "v2.0.0"), std::cmp::Ordering::Less);
        assert_eq!(version_cmp("10.0.0", "9.0.0"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_gen_main_odin() {
        let result = gen_main_odin("myproject");
        assert!(result.contains("package main"));
        assert!(result.contains("Hellope, myproject!"));
        let result2 = gen_main_odin("test");
        assert!(result2.contains("Hellope, test!"));
    }

    #[test]
    fn test_short() {
        assert_eq!(short("abcd1234efgh5678"), "abcd123");
        assert_eq!(short("abc"), "abc");
        assert_eq!(short(""), "");
        assert_eq!(short("1234567890abcdef"), "1234567");
    }
}
