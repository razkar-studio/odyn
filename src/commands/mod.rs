mod get;
mod init;
mod remove;
mod status;
mod sync;
mod update;
mod update_self;
mod version;

use std::path::PathBuf;

use anyhow::{Result, anyhow};

pub use get::cmd_get;
pub use init::cmd_init;
pub use remove::cmd_remove;
pub use status::cmd_status;
pub use sync::cmd_sync;
pub use update::cmd_update;
pub use update_self::cmd_update_self;
pub use version::cmd_version;

pub fn parse_version(s: &str) -> (u32, u32, u32) {
    let s = s.trim_start_matches('v');
    let parts: Vec<u32> = s
        .split('.')
        .map(|p| {
            p.chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap_or(0)
        })
        .collect();
    match parts.as_slice() {
        [major, minor, patch, ..] => (*major, *minor, *patch),
        [major, minor] => (*major, *minor, 0),
        [major] => (*major, 0, 0),
        _ => (0, 0, 0),
    }
}

pub fn version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let (a_major, a_minor, a_patch) = parse_version(a);
    let (b_major, b_minor, b_patch) = parse_version(b);
    (a_major, a_minor, a_patch).cmp(&(b_major, b_minor, b_patch))
}

fn short(commit: &str) -> &str {
    let end = commit
        .char_indices()
        .nth(8)
        .map_or(commit.len(), |(i, _)| i);
    &commit[..end]
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

fn git_head_and_dirty(dep_path: &PathBuf) -> Result<(String, bool)> {
    let output = std::process::Command::new("git")
        .args(["status", "--porcelain=v2", "--branch"])
        .current_dir(dep_path)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("git status failed in '{}'", dep_path.display()));
    }
    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| anyhow!("git output was not valid UTF-8: {e}"))?;
    let commit = stdout
        .lines()
        .find(|l| l.starts_with("# branch.oid "))
        .and_then(|l| l.strip_prefix("# branch.oid "))
        .ok_or_else(|| {
            anyhow!(
                "could not find commit in git status output for '{}'",
                dep_path.display()
            )
        })?
        .trim()
        .to_string();
    let dirty = stdout.lines().any(|l| !l.starts_with('#'));
    Ok((commit, dirty))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::gen_main_odin;

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
        assert_eq!(parse_version("0.3.1-beta"), (0, 3, 1));
        assert_eq!(parse_version("1.0.0-rc.1"), (1, 0, 0));
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
        assert_eq!(short("abcd1234efgh5678"), "abcd1234");
        assert_eq!(short("abc"), "abc");
        assert_eq!(short(""), "");
        assert_eq!(short("1234567890abcdef"), "12345678");
    }
}
