use super::{
    super::{context::*, url::*},
    http_url::*,
};

impl UrlContext {
    /// Construct a [HttpUrl].
    pub fn http_url(self: &UrlContextRef, url: url::Url) -> UrlRef {
        HttpUrl::new(self, url).into()
    }
}
