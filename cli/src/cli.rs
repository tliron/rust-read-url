use {
    clap::{builder::*, *},
    kutil_cli::clap::*,
    read_url::*,
    std::path::*,
};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// Read URLs
#[derive(Parser)]
#[command(
    name = "read-url",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = clap_styles())
]
pub struct CLI {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// can be a URL or a file path
    #[arg(verbatim_doc_comment)]
    pub input_url_or_path: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_path: Option<PathBuf>,

    /// cache base directory
    #[arg(long = "cache", short = 'c', default_value = UrlCache::default_base_directory().into_os_string(), verbatim_doc_comment)]
    pub cache: PathBuf,

    /// use asynchronous I/O
    #[arg(long = "async", short = 'a', verbatim_doc_comment)]
    pub asynchronous: bool,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum)]
    pub colorize: Colorize,

    /// suppress console output
    #[arg(long, short = 'q', verbatim_doc_comment)]
    pub quiet: bool,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub verbose: u8,

    /// log to file path;
    /// defaults to stderr
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub log_path: Option<PathBuf>,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// SubCommands
//

// TODO: subcommands don't work with arg

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// show the version of read-url
    #[command(action = ArgAction::Version)]
    Version(Version),

    /// output the shell auto-completion script
    Completion(Completion),
}
