use super::super::{context::*, url::*, util::*};

use std::{collections::*, fmt, path::*};

//
// FileUrl
//

/// A URL for a file or a directory that is locally accessible.
///
/// The URL scheme is "file:".
///
/// A conforming file URL will have a canonical path (no "." or ".." segments).
/// If it's a directory then it will end in a path separator.
///
/// The path notation depends on the operating system on which this library is
/// compiled. For example, on Unix-like operating systems the path separator would
/// be "/" and the root would start with "/",  while on Windows the path separator
/// would be "\" and the root would start with the drive name and ":\", e.g. "C:\".
///
/// However, note that the URL notation is standardized differently. Only "/" is
/// used as a path separator and for the root. Thus, on Windows "\" becomes a "/".
/// Preceding the path is "//" plus a host. Because the host is rarely used, file
/// URLs most often start with "file:///".
///
/// For example, on Windows the path "C:\Windows\win.ini" would have the URL
/// "file:///C:/Windows/win.ini".
///
/// This library supports the host for presentation purposes, but it is not
/// used for [URL::open].
#[derive(Debug, Clone)]
pub struct FileUrl {
    /// The [PathBuf].
    pub path: PathBuf,

    /// The optional host (for representation purposes only).
    pub host: Option<String>,

    /// The optional query.
    pub query: Option<HashMap<String, String>>,

    /// The optional fragment.
    pub fragment: Option<String>,

    pub(crate) context: UrlContextRef,
}

impl FileUrl {
    /// Constructor.
    pub fn new(
        context: &UrlContextRef,
        path: PathBuf,
        host: Option<String>,
        query: Option<HashMap<String, String>>,
        fragment: Option<String>,
    ) -> Self {
        Self { host, path, query, fragment, context: context.clone() }
    }

    /// Constructor.
    pub fn new_with(&self, path: PathBuf) -> Self {
        Self::new(&self.context, path, self.host.clone(), self.query.clone(), self.fragment.clone())
    }
}

impl fmt::Display for FileUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = match &self.host {
            Some(host) => host,
            None => "",
        };

        let query = url_query_string(&self.query);
        let fragment = url_fragment_string(&self.fragment);

        write!(formatter, "file://{}{}{}{}", host, self.path.display(), query, fragment)
    }
}

// Conversions

impl Into<UrlRef> for FileUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
