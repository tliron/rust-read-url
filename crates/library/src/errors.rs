use {
    std::{fmt, io, sync::*},
    thiserror::*,
};

//
// UrlError
//

/// Common error for read-url APIs.
#[derive(Debug, Error)]
pub enum UrlError {
    /// Unsupported scheme.
    #[error("unsupported scheme: {0}")]
    UnsupportedScheme(String),

    /// Unsupported format.
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Malformed URL.
    #[error("malformed URL: {0}")]
    MalformedUrl(String),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// I/O many.
    #[error("I/O: {0:?}")]
    IoMany(Vec<io::Error>),

    /// Concurrency.
    #[error("concurrency: {0}")]
    Concurrency(String),

    /// Reqwest.
    #[cfg(feature = "http")]
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Git.
    #[cfg(feature = "git")]
    #[error("git: {0}")]
    Git(#[from] super::git::GitError),

    /// Zip.
    #[cfg(feature = "zip")]
    #[error("Zip: {0}")]
    Zip(#[from] rc_zip_sync::rc_zip::error::Error),
}

impl UrlError {
    /// I/O error: not found.
    pub fn new_io_not_found<UrlT>(url: UrlT) -> UrlError
    where
        UrlT: fmt::Display,
    {
        io::Error::new(io::ErrorKind::NotFound, format!("not found: {}", url)).into()
    }
}

impl<GuardT> From<PoisonError<GuardT>> for UrlError {
    fn from(value: PoisonError<GuardT>) -> Self {
        Self::Concurrency(value.to_string())
    }
}
