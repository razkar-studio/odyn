fn main() {
    let method = std::env::var("ODYN_INSTALL_METHOD").unwrap_or_else(|_| {
        let is_git = std::path::Path::new(".git").exists();

        if is_git {
            "source".to_string()
        } else {
            "cargo".to_string()
        }
    });

    println!("cargo:rustc-env=ODYN_INSTALL_METHOD={}", method);

    if method == "source" {
        let hash = std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        println!("cargo:rustc-env=ODYN_GIT_HASH={}", hash);
    }

    let build_date = std::process::Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            std::process::Command::new("powershell")
                .args(["-Command", "Get-Date -Format 'yyyy-MM-dd'"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        });

    println!("cargo:rustc-env=ODYN_BUILD_DATE={}", build_date);
    println!("cargo:rerun-if-env-changed=ODYN_INSTALL_METHOD");
}
