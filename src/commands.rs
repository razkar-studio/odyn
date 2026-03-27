use std::path::PathBuf;

use crate::{
    constants::{DEPS_DIR, OLS_JSON},
    storage::{
        Dep, DepState, Lockfile, check_git, gen_main_odin, load_lockfile, save_lockfile,
        save_lockfile_at,
    },
};
use anyhow::{Result, anyhow};
use farben::{ceprintln, cprintln};

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
    if !no_src {
        std::fs::create_dir_all(root.join("src"))?;
        std::fs::write(
            root.join("src").join("main.odin"),
            gen_main_odin(&project_name),
        )?;
    } else {
        std::fs::write(root.join("main.odin"), gen_main_odin(&project_name))?;
    }

    if with_readme {
        std::fs::write(root.join("README.md"), format!("# {project_name}\n"))?;
    }

    std::fs::write(root.join("ols.json"), OLS_JSON)?;

    save_lockfile_at(&Lockfile { dep: Vec::new() }, &root)?;
    let license_content: String = match license.to_lowercase().as_str() {
        "mit" => "MIT License\n\nCopyright (c) ...\n".to_string(),
        "apache" => "Apache License 2.0\n\n...".to_string(),
        other => format!("License: {other}\n"),
    };

    std::fs::write(root.join("LICENSE"), license_content)?;
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

pub(crate) fn cmd_sync() -> Result<()> {
    check_git()?;
    let lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.is_empty() {
        return Ok(());
    }

    let mut result: Vec<(&Dep, DepState)> = Vec::new();
    for dep in &lockfile.dep {
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        if !dep_path.exists() {
            result.push((dep, DepState::Missing))
        } else {
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
                result.push((dep, DepState::Modified { actual: commit }))
            }
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
        ceprintln!("[error]       error[/] some deps have local changes:");
        for (dep, actual) in modified {
            ceprintln!(
                "[error]       error[/] '{}': expected '{}' but found '{}'",
                dep.name,
                &dep.commit[..7],
                &actual[..7]
            );
        }
        return Err(anyhow!("sync failed: modified dependencies found"));
    }

    for (dep, state) in &result {
        let dep_path: PathBuf = PathBuf::from(DEPS_DIR).join(&dep.name);
        match state {
            DepState::Missing => {
                cprintln!("[load]{:>12}[/] '{}', cloning...", "Syncing", dep.name);
                let clone_status = std::process::Command::new("git")
                    .arg("clone")
                    .arg(&dep.source)
                    .arg(&dep_path)
                    .status()?;
                if !clone_status.success() {
                    return Err(anyhow!("failed to clone '{}'", dep.name));
                }
                let reset_status = std::process::Command::new("git")
                    .args(["reset", "--hard", dep.commit.as_str()])
                    .current_dir(&dep_path)
                    .status()?;
                if !reset_status.success() {
                    return Err(anyhow!("failed to reset '{}' to pinned commit", dep.name));
                }
                cprintln!("[load]{:>12}[/] '{}'", "Synced", dep.name);
            }
            DepState::Ok => {
                cprintln!("[load]{:>12}[/] '{}'", "Verified", dep.name);
            }
            _ => {}
        }
    }

    cprintln!("[load]{:>12}[/] all dependencies up to date", "Finished");
    Ok(())
}

pub(crate) fn cmd_get(source: String, name: Option<String>) -> Result<()> {
    let name: String =
        name.unwrap_or_else(|| source.split('/').last().unwrap_or("unknown").to_string());
    let name: String = name.strip_suffix(".git").unwrap_or(&name).to_string();

    check_git()?;
    let mut lockfile: Lockfile = load_lockfile()?;

    if lockfile.dep.iter().any(|dep| dep.source == source) {
        cprintln!(
            "[load]{:>12}[/] '{}' already exists, skipping",
            "Skipping",
            name
        );
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
