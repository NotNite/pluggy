use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub enum Args {
    /// Manage a custom repository.
    #[clap(subcommand, verbatim_doc_comment)]
    Official(RepoCommand),

    /// Manage the official repository.
    #[clap(subcommand, verbatim_doc_comment)]
    Custom(RepoCommand),
}

#[derive(Debug, Parser)]
pub enum RepoCommand {
    /// Initialize a new repository.
    /// When used for the official repository, it is cloned locally.
    /// When used for a custom repository, a manifest file is created in the current directory.
    #[clap(verbatim_doc_comment)]
    Init,

    /// Adds or updates a plugin to the repository.
    /// When used for the official repository, a manifest file is generated in a new branch.
    /// When used for a custom repository, adds the plugin to the manifest file.
    #[clap(verbatim_doc_comment)]
    Add {
        /// The internal name of the plugin.
        #[clap(verbatim_doc_comment)]
        name: String,

        /// A Git remote.
        #[clap(verbatim_doc_comment)]
        remote: String,

        /// The path to the project in the remote.
        /// Defaults to the internal name.
        #[clap(short, long, verbatim_doc_comment)]
        path: Option<String>,

        /// The branch to use, defaulting to the default branch of the remote.
        /// Has no effect when commit is specified.
        #[clap(short, long, verbatim_doc_comment)]
        branch: Option<String>,

        /// The commit to use, defaulting to the latest commit on the specified branch.
        #[clap(short, long, verbatim_doc_comment)]
        commit: Option<String>,

        /// Whether to save the plugin in plugin testing.
        /// Defaults to true on the official repository, and false on custom repositories.
        #[clap(short, long, verbatim_doc_comment)]
        testing: bool,

        /// What track to use.
        /// Only applies to the official repository.
        /// Defaults to 'stable' or 'testing/live' depending on the testing flag.
        #[clap(long, verbatim_doc_comment)]
        track: Option<String>,

        /// What owners can control the plugin, separated by commas.
        /// Must be specified on the official repository.
        /// This has no effect on custom repositories.
        #[clap(short, long, verbatim_doc_comment)]
        owners: Option<Vec<String>>,
    },

    /// Pushes changes made to the repository.
    /// When used on the official repository, all staged changes are pushed to their respective branches.
    /// When used on a custom repository, the configured push target is executed.
    #[clap(verbatim_doc_comment)]
    Push {
        /// The plugin to push changes for.
        /// Only applies to the official repository.
        name: Option<String>,
    },
}
