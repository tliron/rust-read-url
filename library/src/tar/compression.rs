use super::super::errors::*;

use {kutil::std::*, std::str::*};

//
// TarCompression
//

/// Tar compression.
#[derive(Clone, Copy, Debug, Display)]
#[display(lowercase)]
pub enum TarCompression {
    /// No compression.
    None,

    /// GZip compression.
    GZip,

    /// Zstandard compression.
    #[strings("zstd")]
    Zstandard,
}

impl FromStr for TarCompression {
    type Err = UrlError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation {
            "gzip" => Ok(Self::GZip),
            "zstd" => Ok(Self::Zstandard),
            _ => Err(UrlError::UnsupportedFormat(representation.into())),
        }
    }
}
