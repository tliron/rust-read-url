use super::{
    super::{context::*, url::*},
    mock_url::*,
};

impl UrlContext {
    /// Construct a [MockUrl].
    pub fn mock_url(
        self: &UrlContextRef,
        url_representation: String,
        slashable: bool,
        base_url_representation: Option<String>,
        query: Option<UrlQuery>,
        fragment: Option<String>,
        format: Option<String>,
        content: Option<Vec<u8>>,
    ) -> UrlRef {
        MockUrl::new(self, url_representation, slashable, base_url_representation, query, fragment, format, content)
            .into()
    }
}
