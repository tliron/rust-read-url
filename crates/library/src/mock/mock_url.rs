use super::super::{context::*, url::*, util::*};

use std::{collections::*, fmt};

//
// MockUrl
//

/// A standalone URL implementation intended for testing purposes.
///
/// You can set a URL representation as you please, mocking any other scheme or not
/// following the URL notation at all. Thus mock URLs must be explictly created via
/// [UrlContext::mock_url] and cannot be returned by general [UrlContext] functions.
///
/// The [URL::base] and [URL::relative] functions are supported in two modes. When
/// slashable is true, they will interpret the URL representation as a Unix-style filesystem
/// path, whereby the path separator is "/", and "." and ".." are supported for path
/// traversal. When slashable is false, [URL::relative] does simple string concatenation,
/// and you must explicitly provide a base_url_representation if you want to support [URL::base].
/// For both functions, the content and format are simply cloned.
///
/// [URL::conform] does nothing.
///
/// For custom URLs that are supported by general [UrlContext] functions, see
/// [InternalUrl](super::super::internal::InternalUrl).
#[derive(Clone, Debug)]
pub struct MockUrl {
    /// The URL representation.
    pub url_representation: String,

    /// Whether the URL representation is "slashable".
    pub slashable: bool,

    /// The optional base URL representation (used when slashable is false).
    pub base_url_representation: Option<String>,

    /// The content.
    pub content: Option<Vec<u8>>,

    /// The optional format.
    pub format: Option<String>,

    /// The optional query.
    pub query: Option<HashMap<String, String>>,

    /// The optional fragment.
    pub fragment: Option<String>,

    pub(crate) context: UrlContextRef,
}

impl MockUrl {
    /// Constructor.
    pub fn new(
        context: &UrlContextRef,
        url_representation: String,
        slashable: bool,
        base_url_representation: Option<String>,
        content: Option<Vec<u8>>,
        format: Option<String>,
        query: Option<HashMap<String, String>>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            url_representation,
            slashable,
            base_url_representation,
            content,
            format,
            query,
            fragment,
            context: context.clone(),
        }
    }

    /// Constructor.
    pub fn new_with(&self, url_representation: String) -> MockUrl {
        Self {
            context: self.context.clone(),
            url_representation,
            slashable: self.slashable,
            content: self.content.clone(),
            format: self.format.clone(),
            query: None,
            fragment: None,
            base_url_representation: None,
        }
    }
}

impl fmt::Display for MockUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let query = url_query_string(&self.query);
        let fragment = url_fragment_string(&self.fragment);
        write!(formatter, "{}{}{}", self.url_representation, query, fragment)
    }
}

// Conversions

impl Into<UrlRef> for MockUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
