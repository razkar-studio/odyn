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
    Get {
        /// Git URL or `user/repo` shorthand to the dependency.
        source: String,

        /// Name for the `odyn_deps/` subfolder. Defaults to the repo name.
        name: Option<String>,

        /// Platform to resolve `user/repo` shorthand against.
        #[arg(long, default_value = "github")]
        platform: String,

        /// Pin a specific commit instead of HEAD.
        #[arg(long)]
        commit: Option<String>,

        /// Shallow clone with the given history depth.
        #[arg(long)]
        depth: Option<u32>,

        /// Extra arguments to pass directly to `git clone`.
        #[arg(last = true)]
        extra_args: Vec<String>,
    },

    /// Create a new Odin project with the standard layout.
    Init {
        /// Name of the project directory to create.
        project_name: Option<String>,

        /// License to generate.
        #[arg(long, default_value = "mit")]
        license: String,

        /// Add a `README.md` stub.
        #[arg(long)]
        with_readme: bool,

        /// Skip creating the `src/` directory.
        #[arg(long)]
        no_src: bool,

        /// Migrate an existing Odin project to use Odyn.
        #[arg(long)]
        migrate: bool,
    },

    /// Sync `odyn_deps/` to match Odyn.lock exactly.
    Sync {
        /// Force revert locally modified changes instead of aborting
        #[arg(long)]
        force: bool,

        /// Skip checking a specific dependency entirely.
        #[arg(long)]
        skip: Vec<String>,
    },

    /// Remove a dependency from `odyn_deps/` and Odyn.lock.
    Remove {
        /// Name of the dependency to remove.
        name: String,
    },

    /// Update a dependency to its latest commit and re-pin it.
    Update {
        /// Name of the dependency to update.
        name: String,
    },

    /// Show the current state of all vendored dependencies.
    Status,

    /// Updates Odyn itself to the latest stable release.
    #[command(name = "update-self")]
    UpdateSelf {
        /// Update to the latest pre-release.
        #[arg(long)]
        pre_release: bool,

        /// Update to the latest nightly build.
        #[arg(long)]
        nightly: bool,

        /// Force update to the latest stable release, even if the local version is newer.
        #[arg(long)]
        force_stable: bool,
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
                    status("Creating", "load", &format!("odin project '{name}'"));
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
            Commands::UpdateSelf {
                pre_release,
                nightly,
                force_stable,
            } => {
                cmd_update_self(pre_release, nightly, force_stable)?;
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
