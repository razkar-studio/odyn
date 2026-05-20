use anyhow::{Result, anyhow};
use sha2::{Digest, Sha256};

use crate::constants::VERSION;
use crate::ui::status;

pub fn cmd_update_self(
    pre_release: bool,
    nightly: bool,
    commit_override: Option<String>,
    force_stable: bool,
) -> Result<()> {
    if pre_release && nightly {
        return Err(anyhow!(
            "--pre-release and --nightly cannot be used together"
        ));
    }
    if pre_release && force_stable {
        return Err(anyhow!(
            "--pre-release and --force-stable cannot be used together"
        ));
    }
    if nightly && force_stable {
        return Err(anyhow!(
            "--nightly and --force-stable cannot be used together"
        ));
    }

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    if nightly {
        let commit = if let Some(c) = commit_override {
            c
        } else {
            ureq::get("https://codeberg.org/api/v1/repos/razkar/odyn/branches/main")
                .call()
                .ok()
                .and_then(|mut r| r.body_mut().read_to_string().ok())
                .and_then(|body| {
                    body.split("\"id\":")
                        .nth(1)?
                        .trim_start_matches([' ', '\t'])
                        .strip_prefix('"')?
                        .split('"')
                        .next()
                        .map(|s| s[..8].to_string())
                })
                .unwrap_or_else(|| "unknown".to_string())
        };

        status(
            "Nightly",
            "load",
            &format!("building from commit {commit}..."),
        );

        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let temp_root = std::env::temp_dir().join("odyn-nightly-build");
        if temp_root.exists() {
            std::fs::remove_dir_all(&temp_root).ok();
        }
        std::fs::create_dir_all(&temp_root)?;

        let exit = std::process::Command::new(&cargo)
            .args([
                "install",
                "--git",
                "https://codeberg.org/razkar/odyn.git",
                "--force",
                "--no-default-features",
                "--rev",
                &commit,
                "--root",
            ])
            .arg(&temp_root)
            .env("ODYN_INSTALL_METHOD", "source")
            .status()
            .map_err(|e| anyhow!("failed to run cargo: {e}"))?;

        if !exit.success() {
            return Err(anyhow!("cargo install failed"));
        }

        #[cfg(target_os = "windows")]
        let built_binary = temp_root.join("bin").join("odyn.exe");
        #[cfg(not(target_os = "windows"))]
        let built_binary = temp_root.join("bin").join("odyn");

        if !built_binary.exists() {
            return Err(anyhow!(
                "built binary not found at {}",
                built_binary.display()
            ));
        }

        let current_exe = std::env::current_exe()?;

        #[cfg(target_os = "windows")]
        {
            let old_path = current_exe.with_extension("exe.old");
            if let Err(e) = std::fs::rename(&current_exe, &old_path) {
                std::fs::remove_dir_all(&temp_root).ok();
                return Err(anyhow!("failed to rename current binary: {e}"));
            }
            if let Err(e) = std::fs::copy(&built_binary, &current_exe) {
                std::fs::rename(&old_path, &current_exe).ok();
                std::fs::remove_dir_all(&temp_root).ok();
                return Err(anyhow!("failed to install new binary: {e}"));
            }
            std::fs::remove_dir_all(&temp_root).ok();
            std::fs::remove_file(&old_path).ok();
        }

        #[cfg(not(target_os = "windows"))]
        {
            let old_path = current_exe.with_extension("bak");
            if let Err(e) = std::fs::rename(&current_exe, &old_path) {
                std::fs::remove_dir_all(&temp_root).ok();
                return Err(anyhow!("failed to rename current binary: {e}"));
            }
            if let Err(e) = std::fs::copy(&built_binary, &current_exe) {
                std::fs::rename(&old_path, &current_exe).ok();
                std::fs::remove_dir_all(&temp_root).ok();
                return Err(anyhow!("failed to install new binary: {e}"));
            }
            std::fs::remove_dir_all(&temp_root).ok();
            std::fs::remove_file(&old_path).ok();
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

    if !pre_release && !nightly && !force_stable && super::version_cmp(VERSION, latest).is_gt() {
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
    let actual = hash.iter().map(|b| format!("{b:02x}")).collect::<String>();

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
        let old_path = current_exe.with_extension("bak");
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

    status("Updated", "success", &format!("odyn updated to v{latest}"));
    status("Info", "info", "restart odyn for changes to take effect");

    Ok(())
}
