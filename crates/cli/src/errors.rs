use {kutil_cli::run::*, read_url::*, std::io, thiserror::*};

//
// MainError
//

#[derive(Debug, Error)]
pub enum MainError {
    #[error("exit: {0}")]
    #[allow(unused)]
    Exit(#[from] Exit),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("URL: {0}")]
    Url(#[from] UrlError),
}

impl HasExit for MainError {
    fn get_exit(&self) -> Option<&Exit> {
        if let MainError::Exit(exit) = self {
            Some(exit)
        } else {
            None
        }
    }
}
