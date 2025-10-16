use super::super::{context::*, errors::*, url::*, util::*};

use {
    relative_path::*,
    std::{fmt, sync::*},
};

//
// ZipUrl
//

/// A URL for an entry in a ZIP archive.
///
/// The URL scheme is "zip:", followed by full archive URL, a `!`, and then the entry path
/// within the archive.
#[derive(Clone, Debug)]
pub struct ZipUrl {
    /// The archive [URL].
    pub archive_url: Arc<UrlRef>,

    /// The entry path.
    pub path: RelativePathBuf,

    pub(crate) context: UrlContextRef,
}

impl ZipUrl {
    /// Constructor.
    pub fn new(context: &UrlContextRef, archive_url: Arc<UrlRef>, path: RelativePathBuf) -> Self {
        Self { archive_url, path, context: context.clone() }
    }

    /// Constructor.
    pub fn new_with(&self, path: RelativePathBuf) -> Self {
        Self::new(&self.context, self.archive_url.clone(), path)
    }

    /// Parse.
    pub fn parse(url_representation: &str) -> Result<(String, String), UrlError> {
        parse_archive_entry_url_representation(url_representation, "zip")
    }
}

impl fmt::Display for ZipUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "zip:{}!{}", self.archive_url, self.path)
    }
}

// Conversions

impl Into<UrlRef> for ZipUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
