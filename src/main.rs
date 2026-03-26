mod commands;
mod constants;
mod storage;

use clap::{Parser, Subcommand};

/// Odyn — reproducible vendoring for Odin projects.
///
/// Manages dependencies by cloning Git repositories into `vendor/`
/// and pinning exact commits in `Odyn.lock`. No registry, no solver,
/// no magic.
#[derive(Parser)]
#[command(name = "odyn", about = "Reproducible vendoring for Odin projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a dependency into vendor/ and pin it in Odyn.lock.
    ///
    /// Clones the repository at SOURCE and checks out its current HEAD.
    /// The resulting folder is placed at vendor/<name>, where <name>
    /// defaults to the repository name if not specified.
    Get {
        /// Git URL or local path to the dependency.
        source: String,

        /// Name for the vendor/ subfolder. Defaults to the repo name.
        name: Option<String>,
    },

    /// Create a new Odin project with the standard layout.
    ///
    /// Scaffolds a new project directory containing `src/main.odin`,
    /// an empty `vendor/`, and an empty `Odyn.lock`. Pass flags to
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

    /// Sync vendor/ to match Odyn.lock exactly.
    ///
    /// Re-clones missing dependencies and resets existing ones to their
    /// pinned commits. Errors if any vendor/ folder has uncommitted
    /// local changes. Safe to run multiple times — always produces
    /// the same result.
    Sync,

    /// Remove a dependency from vendor/ and Odyn.lock.
    ///
    /// Deletes vendor/<name> and strips the corresponding entry from
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
    /// affected — no transitive updates.
    Update {
        /// Name of the dependency to update, as it appears in Odyn.lock.
        name: String,
    },

    /// Show the current state of all vendored dependencies.
    ///
    /// Checks each entry in Odyn.lock against its folder in vendor/
    /// and reports whether it is clean, missing, or modified.
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { source, name } => {
            println!("get: {} as {:?}", source, name);
        }
        Commands::Sync => {
            println!("sync");
        }
        Commands::Remove { name } => {
            println!("remove: {}", name);
        }
        Commands::Update { name } => {
            println!("update: {}", name);
        }
        Commands::Status => {
            println!("status");
        }
        Commands::Init {
            project_name,
            license,
            with_readme,
            no_src,
        } => {
            println!("init: {project_name} with license {license}");
            if with_readme {
                println!("init: with readme");
            }
            if no_src {
                println!("init: without src/ directory")
            }
        }
    }
}
