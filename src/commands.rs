use std::path::PathBuf;

use crate::storage::{Dep, Lockfile, check_git, load_lockfile};
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

    let exit_status = std::process::Command::new("git")
        .arg("clone")
        .arg(&source)
        .arg(PathBuf::from(format!("vendor/{name}")))
        .status()?;

    if !exit_status.success() {
        return Err(anyhow!("git clone failed"));
    }

    lockfile.dep.push(Dep {
        name: name.clone(),
        source,
        commit: String::new(), // we still need to grab the HEAD commit SHA
    });

    // save lockfile here

    Ok(())
}
