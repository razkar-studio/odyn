use std::path::PathBuf;

use crate::{
    constants::DEPS_DIR,
    storage::{Dep, Lockfile, check_git, load_lockfile, save_lockfile},
};
use anyhow::{Result, anyhow};

pub(crate) fn cmd_get(source: String, name: Option<String>) -> Result<()> {
    let name: String =
        name.unwrap_or_else(|| source.split('/').last().unwrap_or("unknown").to_string());
    let name: String = name.strip_suffix(".git").unwrap_or(&name).to_string();

    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.iter().any(|dep| dep.source == source) {
        eprintln!("warn: library already exists");
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

    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(&dep_path)
        .output()?;
    let commit = String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?
        .trim()
        .to_string();

    lockfile.dep.push(Dep {
        name: name.clone(),
        source,
        commit,
    });

    save_lockfile(&lockfile)?;

    Ok(())
}
