use farben::cprintln;

use crate::constants::VERSION;

pub fn cmd_version(verbose: bool) {
    let special = match env!("ODYN_INSTALL_METHOD") {
        "cargo" => "[ansi(173)]Cargo Edition".to_string(),
        "source" => {
            let res = "[ansi(62)]Nightly".to_string();
            match option_env!("ODYN_GIT_HASH") {
                None => res,
                Some(hash) => {
                    let mut res = res;
                    res.push_str(&format!("[/ansi(62)], commit {hash}"));
                    res
                }
            }
        }
        "release" => {
            let mut res = String::new();
            res.push_str(match std::env::consts::OS {
                "linux" => "[yellow]Linux[/yellow]",
                "windows" => "[blue]Windows[/blue]",
                "macos" => "[ansi(250)]macOS[/ansi(250)]",
                "android" => "[green]Android[/green]",
                "freebsd" => "[bright-red]FreeBSD[/bright-red]",
                "netbsd" => "[ansi(214)]NetBSD[/ansi(214)]",
                other => other,
            });
            res.push(' ');
            res.push_str(std::env::consts::ARCH);
            res
        }
        _ => String::new(),
    };

    let os_arch = if env!("ODYN_INSTALL_METHOD") == "release" {
        String::new()
    } else {
        let os_name = match std::env::consts::OS {
            "linux" => "[yellow]Linux[/yellow]",
            "windows" => "[blue]Windows[/blue]",
            "macos" => "[ansi(250)]macOS[/ansi(250)]",
            "android" => "[green]Android[/green]",
            "freebsd" => "[bright-red]FreeBSD[/bright-red]",
            "netbsd" => "[ansi(214)]NetBSD[/ansi(214)]",
            other => other,
        };
        format!("| {os_name} {}", std::env::consts::ARCH)
    };

    let line = if os_arch.is_empty() {
        format!("[bold blue]Odyn[/blue] v{VERSION} {special}")
    } else {
        format!("[bold blue]Odyn[/blue] v{VERSION} {special} {os_arch}")
    };
    cprintln!("{line}");
    println!("    Reproducible vendoring tool for the Odin programming language.");

    let git_version = std::process::Command::new("git")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    match git_version {
        Some(v) => cprintln!("    [ansi(214)]{}[/ansi(214)][/bold]", v),
        None => cprintln!("    [bright-red]Git Not Installed[/bright-red]"),
    }

    if verbose {
        let install_path = std::env::current_exe()
            .map_or_else(|_| "unknown".to_string(), |p| p.display().to_string());
        cprintln!("    [dim]installed at {}[/dim]", install_path);
        cprintln!("    [dim]built on {}[/dim]", env!("ODYN_BUILD_DATE"));
    }
}
