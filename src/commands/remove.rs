use std::path::PathBuf;

use anyhow::Result;

use crate::constants::DEPS_DIR;
use crate::storage::{Lockfile, load_lockfile, save_lockfile};

pub fn cmd_remove(name: String) -> Result<()> {
    let mut lockfile: Lockfile = load_lockfile()?;
    if !lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow::anyhow!("dependency '{name}' not found in Odyn.lock"));
    }

    let dep_path = PathBuf::from(DEPS_DIR).join(&name);
    if dep_path.exists() {
        std::fs::remove_dir_all(&dep_path)?;
    }

    lockfile.dep.retain(|d| d.name != name);

    save_lockfile(&lockfile)?;
    Ok(())
}
