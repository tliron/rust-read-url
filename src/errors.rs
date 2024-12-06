use {std::io, thiserror::*, url};

//
// UrlError
//

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("unsupported scheme: {0}")]
    UnsupportedScheme(String),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("URL: {0}")]
    URL(#[from] url::ParseError),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl UrlError {
    pub fn new_not_found(url: &str) -> UrlError {
        io::Error::new(io::ErrorKind::NotFound, format!("not found: {}", url)).into()
    }
}
