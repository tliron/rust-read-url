use super::{
    super::{context::*, errors::*, url::*, util::*},
    compression::*,
};

use {
    relative_path::*,
    std::{fmt, sync::*},
};

//
// TarUrl
//

/// A URL for an entry in a tarball (tar archive).
///
/// Supports bare as well as compressed tarballs.
///
/// The URL scheme is "tar:", followed by full archive URL, a `!`, and then the entry path
/// within the archive. The fragment of the archive URL is used to explicitly set the
/// compression algorithm. If the compression is not explicity set, will attempt to determine
/// it according to the format of the archive URL.
#[derive(Clone, Debug)]
pub struct TarUrl {
    /// The archive [URL].
    pub archive_url: Arc<UrlRef>,

    /// The entry path.
    pub path: RelativePathBuf,

    /// Compression.
    pub compression: Option<TarCompression>,

    pub(crate) context: UrlContextRef,
}

impl TarUrl {
    /// Parse.
    pub fn parse(url_representation: &str) -> Result<(String, String), UrlError> {
        parse_archive_entry_url_representation(url_representation, "tar")
    }

    /// Compression from archive URL fragment.
    pub fn compression_from(archive_url: &UrlRef) -> Result<Option<TarCompression>, UrlError> {
        Ok(match archive_url.fragment() {
            Some(fragment) => Some(fragment.parse()?),
            None => None,
        })
    }

    /// Constructor.
    pub fn new(
        context: &UrlContextRef,
        archive_url: Arc<UrlRef>,
        path: RelativePathBuf,
        compression: Option<TarCompression>,
    ) -> Self {
        Self { archive_url, path, compression, context: context.clone() }
    }

    /// Constructor.
    pub fn new_with(&self, path: RelativePathBuf) -> TarUrl {
        Self::new(&self.context, self.archive_url.clone(), path, self.compression.clone())
    }

    #[cfg(any(feature = "blocking", feature = "async"))]
    pub(crate) fn get_compression(&self) -> TarCompression {
        match &self.compression {
            Some(compression) => compression.clone(),

            None => match self.archive_url.format() {
                Some(archive_format) => match archive_format.as_str() {
                    "tar.gz" => TarCompression::GZip,
                    "tar.zstd" => TarCompression::Zstandard,
                    _ => TarCompression::None,
                },

                _ => TarCompression::None,
            },
        }
    }
}

impl fmt::Display for TarUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "tar:{}!{}", self.archive_url, self.path)
    }
}

// Conversions

impl Into<UrlRef> for TarUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
