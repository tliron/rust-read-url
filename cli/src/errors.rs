use {kutil::cli::run::*, read_url::*, std::io, thiserror::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("URL: {0}")]
    URL(#[from] UrlError),

    #[error("missing: {0}")]
    Missing(String),
}

impl RunError for MainError {}
