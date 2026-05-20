use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::DEPS_DIR;
use crate::storage::{Dep, Lockfile, check_git, load_lockfile, save_lockfile};
use crate::ui::status;

use super::git_head;

pub fn cmd_get(
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
        name: name,
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
