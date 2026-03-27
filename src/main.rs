mod commands;
mod constants;
mod storage;
mod ui;

use clap::{Parser, Subcommand};

use ui::{init_styles, status};

use crate::commands::{
    cmd_get, cmd_init, cmd_remove, cmd_status, cmd_sync, cmd_update, cmd_update_self,
};

/// Odyn: reproducible vendoring for Odin projects.
///
/// Manages dependencies by cloning Git repositories into `odyn_deps/`
/// and pinning exact commits in `Odyn.lock`.
#[derive(Parser)]
#[command(name = "odyn", about = "Reproducible vendoring for Odin projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a dependency into odyn_deps/ and pin it in Odyn.lock.
    ///
    /// Clones the repository at SOURCE and checks out its current HEAD.
    /// The resulting folder is placed at odyn_deps/<name>, where <name>
    /// defaults to the repository name if not specified.
    Get {
        /// Git URL or user/repo shorthand to the dependency.
        source: String,

        /// Name for the odyn_deps/ subfolder. Defaults to the repo name.
        name: Option<String>,

        /// Platform to resolve user/repo shorthand against.
        /// Defaults to github. Options: github, codeberg, gitlab, sourcehut
        #[arg(long, default_value = "github")]
        platform: String,
    },

    /// Create a new Odin project with the standard layout.
    ///
    /// Scaffolds a new project directory containing `src/main.odin`,
    /// an empty `odyn_deps/`, and an empty `Odyn.lock`. Pass flags to
    /// adjust what gets generated.
    Init {
        /// Name of the project directory to create.
        project_name: String,

        /// SPDX license identifier to use for the LICENSE file.
        #[arg(long, default_value = "mit")]
        license: String,

        /// Add a README.md stub to the project root.
        #[arg(long)]
        with_readme: bool,

        /// Skip creating the src/ directory.
        #[arg(long)]
        no_src: bool,
    },

    /// Sync odyn_deps/ to match Odyn.lock exactly.
    ///
    /// Re-clones missing dependencies and resets existing ones to their
    /// pinned commits. Errors if any odyn_deps/ folder has uncommitted
    /// local changes. Safe to run multiple times — always produces
    /// the same result.
    Sync,

    /// Remove a dependency from odyn_deps/ and Odyn.lock.
    ///
    /// Deletes odyn_deps/<name> and strips the corresponding entry from
    /// Odyn.lock. Does not touch other dependencies, even if they
    /// share a transitive source.
    Remove {
        /// Name of the dependency to remove, as it appears in Odyn.lock.
        name: String,
    },

    /// Update a dependency to its latest commit and re-pin it.
    ///
    /// Fetches the latest commit from the dependency's source URL and
    /// updates the entry in Odyn.lock. Only the named dependency is
    /// affected, no transitive updates.
    Update {
        /// Name of the dependency to update, as it appears in Odyn.lock.
        name: String,
    },

    /// Show the current state of all vendored dependencies.
    ///
    /// Checks each entry in Odyn.lock against its folder in odyn_deps/
    /// and reports whether it is clean, missing, or modified.
    Status,

    /// Updates Odyn itself to the latest version.
    ///
    /// Fetches the latest release from Odyn's Codeberg repository
    /// and updates Odyn itself.
    #[command(name = "update-self")]
    UpdateSelf,
}

fn main() {
    init_styles();
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        status("Error", "error", &e.to_string());
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Get {
            source,
            name,
            platform,
        } => {
            status("Getting", "load", &format!("'{source}'"));
            cmd_get(source, name, platform)?;
            status("Done", "success", "dependency added");
        }
        Commands::Init {
            project_name,
            license,
            with_readme,
            no_src,
        } => {
            status(
                "Creating",
                "load",
                &format!("odin project '{project_name}'"),
            );
            cmd_init(project_name.clone(), license, with_readme, no_src)?;
            status("Created", "success", &format!("'{project_name}'"));
        }
        Commands::Sync => {
            cmd_sync()?;
        }
        Commands::Remove { name } => {
            status("Removing", "load", &format!("'{name}'"));
            cmd_remove(name)?;
            status("Removed", "success", "dependency removed");
        }
        Commands::Update { name } => {
            cmd_update(name)?;
        }
        Commands::UpdateSelf => {
            cmd_update_self()?;
        }
        Commands::Status => {
            cmd_status()?;
        }
    }
    Ok(())
}
