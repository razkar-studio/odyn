use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::DEPS_DIR;
use crate::storage::{Lockfile, check_git, load_lockfile, save_lockfile};
use crate::ui::status;

use super::{git_head, short};

pub fn cmd_update(name: String) -> Result<()> {
    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;
    if !lockfile.dep.iter().any(|dep| dep.name == name) {
        return Err(anyhow!("dep '{name}' does not exist"));
    }
    let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&name);

    status("Updating", "load", &format!("'{name}'"));

    let is_shallow = dep_path.join(".git").join("shallow").exists();

    let fetch_args: Vec<&str> = if is_shallow {
        vec!["fetch", "origin", "--quiet", "--unshallow"]
    } else {
        vec!["fetch", "origin", "--quiet"]
    };
    let fetch_status = std::process::Command::new("git")
        .args(&fetch_args)
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
