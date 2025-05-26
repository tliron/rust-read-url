use super::super::errors::*;

use std::{fmt, str::*};

//
// TarCompression
//

/// Tar compression.
#[derive(Clone, Copy, Debug)]
pub enum TarCompression {
    /// No compression.
    None,

    /// GZip compression.
    GZip,

    /// Zstd compression.
    Zstd,
}

impl FromStr for TarCompression {
    type Err = UrlError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation {
            "gzip" => Ok(Self::GZip),
            "zstd" => Ok(Self::Zstd),
            _ => Err(UrlError::UnsupportedFormat(representation.into())),
        }
    }
}

impl fmt::Display for TarCompression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::None => "none",
                Self::GZip => "gzip",
                Self::Zstd => "zstd",
            },
            formatter,
        )
    }
}
