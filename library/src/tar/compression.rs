use super::super::errors::*;

use {
    kutil::std::*,
    problemo::{common::*, *},
    std::str::*,
};

//
// TarCompression
//

/// Tar compression.
#[derive(Clone, Copy, Debug, Display)]
#[display(lowercase)]
pub enum TarCompression {
    /// No compression.
    None,

    /// Gzip compression.
    Gzip,

    /// Zstandard compression.
    #[strings("zstd")]
    Zstandard,
}

impl FromStr for TarCompression {
    type Err = Problem;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation {
            "gzip" => Ok(Self::Gzip),
            "zstd" => Ok(Self::Zstandard),
            _ => Err(UnsupportedError::new("tar compression format")
                .into_problem()
                .with(FormatAttachment::new(representation))
                .via(UrlError)),
        }
    }
}
