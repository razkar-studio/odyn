use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::DEPS_DIR;
use crate::storage::{DepState, Lockfile, check_git, load_lockfile};
use crate::ui::status;

use super::{git_head_and_dirty, short};

pub fn cmd_sync(force: bool, skip: Vec<String>) -> Result<()> {
    check_git()?;
    let lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.is_empty() {
        status("Sync", "info", "no dependencies to sync");
        return Ok(());
    }

    let mut result: Vec<(&crate::storage::Dep, DepState)> = Vec::new();
    let mut any_skipped = false;

    for dep in &lockfile.dep {
        if skip.contains(&dep.name) {
            any_skipped = true;
            continue;
        }
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        if dep_path.exists() {
            let (commit, dirty) = match git_head_and_dirty(&dep_path) {
                Ok(r) => r,
                Err(e) => {
                    return Err(anyhow!("failed to read commit for '{}': {e}", dep.name));
                }
            };
            if commit == dep.commit {
                if dirty && force {
                    result.push((dep, DepState::Missing));
                } else if dirty {
                    result.push((dep, DepState::Dirty));
                } else {
                    result.push((dep, DepState::Ok));
                }
            } else if force {
                result.push((dep, DepState::Missing));
            } else {
                result.push((dep, DepState::Modified { actual: commit }));
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

    let dirty: Vec<_> = result
        .iter()
        .filter_map(|(dep, state)| {
            if matches!(state, DepState::Dirty) {
                Some(*dep)
            } else {
                None
            }
        })
        .collect();

    if !modified.is_empty() || !dirty.is_empty() {
        farben::ceprintln!("[error]       Error[/] some deps have local changes:");
        for (dep, actual) in &modified {
            farben::ceprintln!(
                "[error]       Error[/] '{}': expected '{}' but found '{}'",
                dep.name,
                short(&dep.commit),
                short(actual)
            );
        }
        for dep in &dirty {
            farben::ceprintln!(
                "[error]       Error[/] '{}': has uncommitted local changes",
                dep.name
            );
        }
        status("Hint", "info", "revert local changes or use --force");
        return Err(anyhow!("sync failed: modified dependencies found"));
    }

    for (dep, state) in &result {
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        match state {
            DepState::Missing => {
                if dep_path.exists() {
                    let is_shallow = dep_path.join(".git").join("shallow").exists();
                    if is_shallow {
                        status(
                            "Fetching",
                            "load",
                            &format!("'{}' with full history...", dep.name),
                        );
                        let fetch_status = std::process::Command::new("git")
                            .args(["fetch", "origin", "--quiet", "--unshallow"])
                            .current_dir(&dep_path)
                            .status();
                        if fetch_status.is_err() || !fetch_status.unwrap().success() {
                            status(
                                "Hint",
                                "info",
                                &format!(
                                    "unshallow failed for '{}', trying shallow fetch of pinned commit",
                                    dep.name
                                ),
                            );
                            std::process::Command::new("git")
                                .args(["fetch", "origin", dep.commit.as_str(), "--quiet"])
                                .current_dir(&dep_path)
                                .status()
                                .ok();
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
                    }
                    status(
                        "Resetting",
                        "load",
                        &format!("'{}' to pinned commit...", dep.name),
                    );
                } else {
                    status("Syncing", "load", &format!("'{}', cloning...", dep.name));
                    let clone_status = std::process::Command::new("git")
                        .arg("clone")
                        .arg(&dep.source)
                        .arg(&dep_path)
                        .status()?;
                    if !clone_status.success() {
                        return Err(anyhow!("failed to clone '{}'", dep.name));
                    }
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
