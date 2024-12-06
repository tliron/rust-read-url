use {std::io, thiserror::*, url};

//
// UrlError
//

/// URL error.
#[derive(Error, Debug)]
pub enum UrlError {
    /// Unsupported scheme.
    #[error("unsupported scheme: {0}")]
    UnsupportedScheme(String),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// URL.
    #[error("URL: {0}")]
    URL(#[from] url::ParseError),

    /// Reqwest.
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl UrlError {
    /// I/O error: not found.
    pub fn new_not_found(url: &str) -> UrlError {
        io::Error::new(io::ErrorKind::NotFound, format!("not found: {}", url)).into()
    }
}
