use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::DEPS_DIR;
use crate::storage::{Lockfile, check_git, load_lockfile};
use crate::ui::status;

use super::{git_head, short};

pub fn cmd_status() -> Result<()> {
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
