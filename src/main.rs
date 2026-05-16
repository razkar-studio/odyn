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
    /// Print the current Odyn version.
    ///
    /// Along with metadata about the install method and build target.
    /// Use the verbose flag to see the full binary path and build date.
    #[arg(long, short = 'V', action = clap::ArgAction::SetTrue)]
    version: bool,

    /// Print extra details about the Odyn installation.
    ///
    /// This includes the full path to the installed binary
    /// and the date this build was compiled. This flag only has an effect
    /// when used together with --version.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a dependency from a remote Git repository into the `odyn_deps` directory and
    /// record its exact commit in the Odyn.lock file.
    ///
    /// The source can be a full Git URL or a shorthand
    /// like user/repo which is resolved against the chosen platform.
    Get {
        /// The Git URL or a user/repo shorthand pointing to the dependency repository.
        ///
        /// When using the shorthand form the platform flag determines which forge to use.
        source: String,

        /// The name of the subfolder created inside `odyn_deps`.
        ///
        /// If omitted the repository name is used automatically after stripping any trailing .git suffix.
        name: Option<String>,

        /// The forge platform used to resolve a user/repo shorthand into a full URL.
        ///
        /// Supported values include github, codeberg, gitlab, sourcehut, bitbucket and
        /// several others.
        #[arg(long, default_value = "github")]
        platform: String,

        /// Pin the clone to a specific commit instead of the default branch HEAD.
        ///
        /// The dependency is checked out to this exact commit after cloning.
        #[arg(long)]
        commit: Option<String>,

        /// Perform a shallow clone that only fetches the given number of commits from
        /// history.
        ///
        /// This reduces download size and clone time for large repositories.
        #[arg(long)]
        depth: Option<u32>,

        /// Additional arguments passed directly through to the underlying git clone
        /// command.
        ///
        /// Use this for advanced clone options that Odyn does not expose as dedicated flags.
        #[arg(last = true)]
        extra_args: Vec<String>,
    },

    /// Create a new Odin project directory with the standard file layout including a
    /// lockfile, package configuration and an optional source tree.
    ///
    /// This is the fastest way to start a fresh project with Odyn already set up.
    Init {
        /// The name of the project directory to create.
        ///
        /// The directory must not already exist and will be created relative to the current working directory.
        project_name: Option<String>,

        /// The software license to generate as a LICENSE file in the project root.
        ///
        /// Common values are mit, apache, gpl3, bsd2, bsd3, mpl2, unlicense, zlib and isc.
        #[arg(long, default_value = "mit")]
        license: String,

        /// Generate a README.md file with the project name as the title.
        ///
        /// This is useful for projects that will be published publicly or shared with others.
        #[arg(long)]
        with_readme: bool,

        /// Skip the creation of the src directory.
        ///
        /// Use this when you prefer a flat layout with the main source file at the project root instead of inside a subfolder.
        #[arg(long)]
        no_src: bool,

        /// Migrate an existing Odin project to use Odyn for dependency management. This
        /// creates the lockfile and adds a deps collection to ols.json if it exists.
        #[arg(long)]
        migrate: bool,
    },

    /// Walk every dependency listed in Odyn.lock and verify that the local copy inside
    /// `odyn_deps` matches the pinned commit exactly.
    ///
    /// Missing dependencies are cloned from scratch
    /// and modified ones are hard reset to the expected state.
    Sync {
        /// Force a hard reset of locally modified dependencies back to their pinned
        /// commits instead of aborting with an error.
        #[arg(long)]
        force: bool,

        /// Skip the named dependency entirely during the sync pass.
        ///
        /// This flag can be repeated to skip multiple dependencies.
        #[arg(long)]
        skip: Vec<String>,
    },

    /// Remove a dependency from both the `odyn_deps` directory on disk and the Odyn.lock
    /// file.
    ///
    /// The dependency folder is deleted recursively and the lockfile entry is
    /// stripped out so it is no longer tracked.
    Remove {
        /// The name of the dependency to remove.
        ///
        /// This must match the name field as it appears in the Odyn.lock file.
        name: String,
    },

    /// Fetch the latest commits for a dependency and reset it to the newest commit on
    /// its default branch.
    ///
    /// The pinned commit in Odyn.lock is updated to reflect the new
    /// state so future syncs reproduce this revision.
    Update {
        /// The name of the dependency to update. This must match the name field as it
        /// appears in the Odyn.lock file.
        name: String,
    },

    /// Check every vendored dependency in `odyn_deps` against the commit recorded in
    /// Odyn.lock and report whether each one is up to date, missing or locally modified.
    ///
    /// This is a read only operation that never changes files on disk.
    Status,

    /// Download and replace the currently running Odyn binary with the latest version
    /// available from the release channel.
    ///
    /// By default this targets the newest stable release
    /// but pre-release and nightly channels are available through flags.
    #[command(name = "update-self")]
    UpdateSelf {
        /// Target the latest pre-release version instead of a stable release.
        ///
        /// Pre-releases may contain experimental features and are not recommended for production use.
        #[arg(long)]
        pre_release: bool,

        /// Build from the latest commit on the main branch and install it as the current
        /// binary.
        ///
        /// This requires a working Rust toolchain with cargo available on PATH.
        #[arg(long)]
        nightly: bool,

        /// Specify a commit to build from when using --nightly. If omitted the latest
        /// commit on the main branch is used automatically.
        #[arg(long, requires = "nightly")]
        commit: Option<String>,

        /// Force an update to the latest stable release even when the locally installed
        /// version is already newer.
        ///
        /// This is useful for downgrading after testing a pre-release or nightly build.
        #[arg(long)]
        force_stable: bool,
    },

    /// Print the current Odyn version.
    ///
    /// Along with metadata about the install method and build target.
    /// Use the verbose flag to see the full binary path and build date.
    Version {
        /// Print extra details including the absolute path to the installed binary and
        /// the date this build was compiled.
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
                commit,
                force_stable,
            } => {
                cmd_update_self(pre_release, nightly, commit, force_stable)?;
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
