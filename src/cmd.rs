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

    /// Adds a plugin to the repository.
    /// When used for the official repository, a manifest file is generated in a new branch.
    /// When used for a custom repository, adds the plugin to the manifest file.
    #[clap(verbatim_doc_comment)]
    Add {
        /// A Git remote.
        #[clap(verbatim_doc_comment)]
        url: String,

        /// The branch to use, defaulting to the default branch of the remote.
        #[clap(short, long, verbatim_doc_comment)]
        branch: Option<String>,

        /// Whether to save the plugin in plugin testing.
        /// Defaults to true on the official repository, and false on custom repositories.
        #[clap(short, long, verbatim_doc_comment)]
        testing: Option<bool>,

        /// What authors can control the plugin, separated by commas.
        /// Must be specified on the official repository.
        /// This has no effect on custom repositories.
        #[clap(short, long, verbatim_doc_comment)]
        authors: Option<Vec<String>>,
    },

    /// Updates a plugin in the repository.
    #[clap(verbatim_doc_comment)]
    Update {
        /// The internal name of the plugin to update.
        #[clap(verbatim_doc_comment)]
        name: String,

        /// What commit to update to.
        /// Defaults to the latest commit of the specified branch.
        #[clap(short, long, verbatim_doc_comment)]
        commit: Option<String>,

        /// What branch to pull the latest commit from.
        /// Unused if commit is specified.
        /// Defaults to the default branch of the remote.
        #[clap(short, long, verbatim_doc_comment)]
        branch: Option<String>,

        /// Whether to mark this update as testing-exclusive.
        /// This has no effect on custom repositories.
        #[clap(short, long, verbatim_doc_comment)]
        testing: bool,
    },

    /// Pushes changes made to the repository.
    /// When used on the official repository, all staged changes are pushed to their respective branches.
    /// When used on a custom repository, the configured push target is executed.
    #[clap(verbatim_doc_comment)]
    Push,
}
