use super::super::errors::*;

use std::fmt;

//
// TarCompression
//

/// Tar compression.
#[derive(Debug, Clone)]
pub enum TarCompression {
    /// No compression.
    None,

    /// GZip compression.
    GZip,

    /// Zstd compression.
    Zstd,
}

impl TryFrom<&str> for TarCompression {
    type Error = UrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "gzip" => Ok(Self::GZip),
            "zstd" => Ok(Self::Zstd),
            _ => Err(UrlError::UnsupportedFormat(value.into())),
        }
    }
}

impl fmt::Display for TarCompression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => fmt::Display::fmt("none", formatter),
            Self::GZip => fmt::Display::fmt("gzip", formatter),
            Self::Zstd => fmt::Display::fmt("zstd", formatter),
        }
    }
}
