use super::{
    super::{context::*, url::*},
    mock_url::*,
};

use std::collections::*;

impl UrlContext {
    /// Construct a [MockUrl].
    pub fn mock_url(
        self: &UrlContextRef,
        url_representation: String,
        slashable: bool,
        base_url_representation: Option<String>,
        content: Option<Vec<u8>>,
        format: Option<String>,
        query: Option<HashMap<String, String>>,
        fragment: Option<String>,
    ) -> UrlRef {
        MockUrl::new(self, url_representation, slashable, base_url_representation, content, format, query, fragment)
            .into()
    }
}
