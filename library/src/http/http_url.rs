use super::super::{context::*, url::*};

use std::fmt;

//
// HttpUrl
//

/// An HTTP URL.
///
/// The URL scheme is either "http:" or "https:".
///
/// The [URL::base] and [URL::relative] functions interpret the path segment of the URL
/// as a Unix-style filesystem path, whereby the path separator is "/", and "." and
/// ".." are supported for path traversal.
#[derive(Clone, Debug)]
pub struct HttpUrl {
    /// The [Url](url::Url).
    pub url: url::Url,

    pub(crate) context: UrlContextRef,
}

impl HttpUrl {
    /// Constructor.
    pub fn new(context: &UrlContextRef, url: url::Url) -> Self {
        Self { url, context: context.clone() }
    }

    /// Constructor.
    pub fn new_with(&self, path: &str) -> Self {
        let mut url = self.url.clone();
        url.set_path(path);
        url.set_query(None);
        url.set_fragment(None);
        Self::new(&self.context, url)
    }
}

impl fmt::Display for HttpUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.url)
    }
}

// Conversions

impl Into<UrlRef> for HttpUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
