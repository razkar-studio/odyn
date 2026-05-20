use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::{
    APACHE_LICENSE, BSD2_LICENSE, BSD3_LICENSE, DEPS_DIR, GPL3_LICENSE, ISC_LICENSE, LOCKFILE,
    MIT_LICENSE, MPL2_LICENSE, OLS_JSON, UNLICENSE, ZLIB_LICENSE,
};
use crate::storage::{Lockfile, gen_main_odin, save_lockfile_at};

pub fn cmd_init(
    project_name: String,
    license: String,
    with_readme: bool,
    no_src: bool,
    migrate: bool,
) -> Result<()> {
    if migrate {
        let deps_dir = PathBuf::from(DEPS_DIR);
        let ols = PathBuf::from("ols.json");
        let lockfile = PathBuf::from(LOCKFILE);

        if deps_dir.exists() {
            return Err(anyhow!("'{DEPS_DIR}' already exists"));
        }
        if lockfile.exists() {
            return Err(anyhow!("'Odyn.lock' already exists"));
        }

        if ols.exists() {
            let content = std::fs::read_to_string(&ols)?;
            let mut json: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| anyhow!("failed to parse ols.json: {e}"))?;
            let collections = json
                .get_mut("collections")
                .and_then(|v| v.as_array_mut())
                .ok_or_else(|| anyhow!("ols.json has no 'collections' array"))?;
            if collections
                .iter()
                .any(|e| e.get("name").and_then(|n| n.as_str()) == Some("deps"))
            {
                return Err(anyhow!("ols.json already has a 'deps' collection"));
            }
            collections.push(serde_json::json!({ "name": "deps", "path": "odyn_deps" }));
            std::fs::write(&ols, serde_json::to_string_pretty(&json)?)?;
        } else {
            std::fs::write(&ols, OLS_JSON)?;
        }

        std::fs::create_dir_all(&deps_dir)?;

        save_lockfile_at(&Lockfile { dep: Vec::new() }, &PathBuf::from("."))?;

        return Ok(());
    }

    let root: PathBuf = PathBuf::from(&project_name);

    if root.exists() {
        return Err(anyhow!("directory '{project_name}' already exists"));
    }

    std::fs::create_dir_all(root.join(DEPS_DIR))?;
    if no_src {
        std::fs::write(root.join("main.odin"), gen_main_odin(&project_name))?;
    } else {
        std::fs::create_dir_all(root.join("src"))?;
        std::fs::write(
            root.join("src").join("main.odin"),
            gen_main_odin(&project_name),
        )?;
    }

    if with_readme {
        std::fs::write(root.join("README.md"), format!("# {project_name}\n"))?;
    }

    std::fs::write(root.join("ols.json"), OLS_JSON)?;

    save_lockfile_at(&Lockfile { dep: Vec::new() }, &root)?;
    let license_content: String = match license.to_lowercase().as_str() {
        "mit" => MIT_LICENSE.to_string(),
        "apache" => APACHE_LICENSE.to_string(),
        "gpl3" => GPL3_LICENSE.to_string(),
        "bsd2" => BSD2_LICENSE.to_string(),
        "bsd3" => BSD3_LICENSE.to_string(),
        "mpl2" => MPL2_LICENSE.to_string(),
        "unlicense" => UNLICENSE.to_string(),
        "zlib" => ZLIB_LICENSE.to_string(),
        "isc" => ISC_LICENSE.to_string(),
        other => format!("License: {other}\n"),
    };

    std::fs::write(root.join("LICENSE"), license_content)?;
    Ok(())
}
