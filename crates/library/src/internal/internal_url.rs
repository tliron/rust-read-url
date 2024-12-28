use super::super::{context::*, url::*, util::*};

use std::{collections::*, fmt};

//
// InternalUrl
//

/// An internal URL.
///
/// The URL scheme is "internal:", followed by a custom path representation.
///
/// The [URL::base] and [URL::relative] functions are supported in two modes. When
/// slashable is true, they will interpret the path representation as a Unix-style filesystem
/// path, whereby the path separator is "/", and "." and ".." are supported for path
/// traversal. When slashable is false, [URL::relative] does simple string concatenation,
/// and you must explicitly register a base_path_representation if you want to support [URL::base].
///
/// [URL::conform] is critical for internal URLs: it makes sure to fill in metadata
/// from the registry.
///
/// If your use case is testing, it could be that [MockUrl](super::super::mock::MockUrl)
/// would be easier to use, as it is not owned by [UrlContext] and can mock
/// any scheme.
#[derive(Clone, Debug)]
pub struct InternalUrl {
    /// The path representation.
    pub path_representation: String,

    /// Whether the path representation is "slashable".
    pub slashable: bool,

    /// The optional base path representation (used when slashable is false).
    pub base_path_representation: Option<String>,

    /// The optional format.
    pub format: Option<String>,

    /// The optional host (for representation purposes only).
    pub host: Option<String>,

    /// The optional query.
    pub query: Option<HashMap<String, String>>,

    /// The optional fragment.
    pub fragment: Option<String>,

    pub(crate) context: UrlContextRef,
}

impl InternalUrl {
    /// Constructor.
    pub fn new(
        context: &UrlContextRef,
        url_representation: String,
        slashable: bool,
        base_url_representation: Option<String>,
        host: Option<String>,
        query: Option<HashMap<String, String>>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            path_representation: url_representation,
            slashable,
            base_path_representation: base_url_representation,
            format: None,
            host,
            query,
            fragment,
            context: context.clone(),
        }
    }

    /// Constructor.
    pub fn new_with(&self, url_representation: String) -> Self {
        Self::new(&self.context, url_representation, self.slashable, None, self.host.clone(), None, None)
    }
}

impl fmt::Display for InternalUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = match &self.host {
            Some(host) => host,
            None => "",
        };

        let query = url_query_string(&self.query);
        let fragment = url_fragment_string(&self.fragment);

        write!(formatter, "internal://{}{}{}{}", host, self.path_representation, query, fragment)
    }
}

// Conversions

impl Into<UrlRef> for InternalUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
