mod commands;
mod constants;
mod storage;
mod ui;

use anyhow::anyhow;
use clap::{Parser, Subcommand};

use ui::{init_styles, status};

use crate::commands::{
    cmd_get, cmd_init, cmd_remove, cmd_status, cmd_sync, cmd_update, cmd_update_self, cmd_version,
};

/// Odyn: reproducible vendoring for Odin projects.
///
/// Manages dependencies by cloning Git repositories into `odyn_deps/`
/// and pinning exact commits in `Odyn.lock`.
#[derive(Parser)]
#[command(
    name = "odyn",
    about = "Reproducible vendoring for Odin projects",
    disable_version_flag = true,
    arg_required_else_help = true
)]
struct Cli {
    #[arg(long, short = 'V', action = clap::ArgAction::SetTrue)]
    version: bool,

    /// Print extra details (install path, build date). Only meaningful with --version.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a dependency into `odyn_deps/` and pin it in Odyn.lock.
    ///
    /// Clones the repository at SOURCE and checks out its current HEAD.
    /// The resulting folder is placed at `odyn_deps/<name>`, where `<name>`
    /// defaults to the repository name if not specified.
    Get {
        /// Git URL or `user/repo` shorthand to the dependency.
        source: String,

        /// Name for the `odyn_deps/` subfolder. Defaults to the repo name.
        name: Option<String>,

        /// Platform to resolve `user/repo` shorthand against. Defaults to github.
        /// Options: github, codeberg, gitlab, sourcehut, bitbucket, framagit,
        /// disroot, notabug, savannah
        #[arg(long, default_value = "github")]
        platform: String,

        /// Pin a specific commit instead of HEAD.
        #[arg(long)]
        commit: Option<String>,

        /// Perform a shallow clone with the given history depth.
        /// Passes `--depth <n>` to git, limiting the number of commits fetched.
        /// Useful for large repositories where full history is not needed.
        #[arg(long)]
        depth: Option<u32>,

        /// Extra arguments to pass directly to `git clone`.
        #[arg(last = true)]
        extra_args: Vec<String>,
    },

    /// Create a new Odin project with the standard layout.
    ///
    /// Scaffolds a new project directory containing `src/main.odin`,
    /// an empty `odyn_deps/`, and an empty `Odyn.lock`. Pass flags to
    /// adjust what gets generated.
    Init {
        /// Name of the project directory to create.
        project_name: Option<String>,

        /// License to generate. Defaults to mit.
        /// Options: mit, apache, gpl3, bsd2, bsd3, mpl2, unlicense, zlib, isc
        #[arg(long, default_value = "mit")]
        license: String,

        /// Add a `README.md` stub to the project root.
        #[arg(long)]
        with_readme: bool,

        /// Skip creating the `src/` directory.
        #[arg(long)]
        no_src: bool,

        /// Migrate an existing Odin project to use Odyn.
        ///
        /// Adds `odyn_deps/`, `ols.json`, and an empty `Odyn.lock` to the
        /// current directory. Does not create `src/` or any other files.
        /// Errors if any of these already exist.
        #[arg(long)]
        migrate: bool,
    },

    /// Sync `odyn_deps/` to match Odyn.lock exactly.
    ///
    /// Re-clones missing dependencies and resets existing ones to their
    /// pinned commits. Errors if any `odyn_deps/` folder has uncommitted
    /// local changes. Safe to run multiple times — always produces
    /// the same result.
    Sync {
        /// Force revert locally modified changes instead of aborting
        #[arg(long)]
        force: bool,

        /// Skip checking a specific dependency entirely. Chainable.
        #[arg(long)]
        skip: Vec<String>,
    },

    /// Remove a dependency from `odyn_deps/` and Odyn.lock.
    ///
    /// Deletes `odyn_deps/<name>` and strips the corresponding entry from
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
    /// Checks each entry in Odyn.lock against its folder in `odyn_deps/`
    /// and reports whether it is clean, missing, or modified.
    Status,

    /// Updates Odyn itself to the latest stable release.
    ///
    /// Downloads the appropriate binary for your platform from
    /// <https://codeberg.org/razkar/odyn/releases> and replaces
    /// the current executable.
    #[command(name = "update-self")]
    UpdateSelf {
        /// Update to the latest pre-release instead of the latest stable.
        #[arg(long)]
        pre_release: bool,

        /// Update to the latest nightly build.
        #[arg(long)]
        nightly: bool,
    },

    /// Print version information.
    Version {
        /// Print extra details: install path and build date.
        #[arg(long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
}

fn main() {
    init_styles();
    let cli = Cli::parse();

    if cli.version {
        cmd_version(cli.verbose);
        return;
    }

    if cli.verbose && cli.command.is_none() {
        status(
            "Error",
            "error",
            "--verbose only makes sense with --version or the 'version' subcommand",
        );
        std::process::exit(1);
    }

    if let Err(e) = run(cli) {
        status("Error", "error", &e.to_string());
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(command) => match command {
            Commands::Get {
                source,
                name,
                platform,
                commit,
                depth,
                extra_args,
            } => {
                status("Getting", "load", &format!("'{source}'"));
                cmd_get(source, name, platform, commit, depth, extra_args)?;
                status("Done", "success", "dependency added");
            }
            Commands::Init {
                project_name,
                license,
                with_readme,
                no_src,
                migrate,
            } => {
                if !migrate && project_name.is_none() {
                    return Err(anyhow!("project name is required. usage: odyn init <name>"));
                }
                let name = project_name.unwrap_or_default();
                if migrate {
                    status("Migrating", "load", "current project");
                } else {
                    status("Creating", "load", &format!("odin project '{name}'",));
                }
                cmd_init(name.clone(), license, with_readme, no_src, migrate)?;
                if migrate {
                    status("Migrated", "success", "project to Odyn");
                } else {
                    status("Created", "success", &format!("'{name}'"));
                }
            }
            Commands::Sync { force, skip } => {
                cmd_sync(force, skip)?;
            }
            Commands::Remove { name } => {
                status("Removing", "load", &format!("'{name}'"));
                cmd_remove(name)?;
                status("Removed", "success", "dependency removed");
            }
            Commands::Update { name } => {
                cmd_update(name)?;
            }
            Commands::UpdateSelf { pre_release, nightly } => {
                cmd_update_self(pre_release, nightly)?;
            }
            Commands::Status => {
                cmd_status()?;
            }
            Commands::Version { verbose } => {
                cmd_version(verbose);
            }
        },
        None => unreachable!(),
    }
    Ok(())
}
