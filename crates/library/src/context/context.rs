use super::super::{cache::*, internal::*, url::*};

use std::{collections::*, path::*, sync::*};

//
// UrlContext
//

/// Common reference type for [UrlContext].
pub type UrlContextRef = Arc<UrlContext>;

/// Context for [URL](super::super::URL).
#[derive(Debug, Clone)]
pub struct UrlContext {
    /// Base URLs.
    pub base_urls: Arc<Vec<UrlRef>>,

    /// URL overrides.
    pub url_overrides: Arc<Mutex<HashMap<String, String>>>,

    /// Cache.
    pub cache: Arc<UrlCache>,

    /// Internal URL registry.
    pub internal_url_registry: Arc<Mutex<HashMap<String, RegisteredInternalUrl>>>,

    /// Common HTTP client.
    ///
    /// Note that we are using the async version of [reqwest::Client] rather than
    /// the blocking version because 1) the blocking version is not supported in WASM, and
    /// 2) we can just use a straightforward blocking wrapper on top of the async client.
    #[cfg(feature = "http")]
    pub http_client: Arc<LazyLock<reqwest::Client>>,
}

impl UrlContext {
    /// Constructor.
    pub fn new() -> UrlContextRef {
        UrlContext::new_for(None)
    }

    /// Constructor.
    pub fn new_for(cache_base_directory: Option<PathBuf>) -> UrlContextRef {
        UrlContext {
            base_urls: Arc::new(Vec::new()),
            url_overrides: Arc::new(HashMap::new().into()),
            cache: Arc::new(UrlCache::new(cache_base_directory)),
            internal_url_registry: Arc::new(HashMap::new().into()),

            #[cfg(feature = "http")]
            http_client: Arc::new(LazyLock::new(|| reqwest::Client::new())),
        }
        .into()
    }

    /// Return a child context with different base URLs.
    ///
    /// The child context shares everything else with the parent.
    pub fn with_base_urls(self: &UrlContextRef, base_urls: Vec<UrlRef>) -> UrlContextRef {
        UrlContext {
            base_urls: Arc::new(base_urls),
            url_overrides: self.url_overrides.clone(),
            cache: self.cache.clone(),
            internal_url_registry: self.internal_url_registry.clone(),

            #[cfg(feature = "http")]
            http_client: self.http_client.clone(),
        }
        .into()
    }

    /// Return a child context with a different cache.
    ///
    /// The child context shares everything else with the parent.
    pub fn with_cache(self: &UrlContextRef, cache_base_directory: Option<PathBuf>) -> UrlContextRef {
        UrlContext {
            base_urls: self.base_urls.clone(),
            url_overrides: self.url_overrides.clone(),
            cache: Arc::new(UrlCache::new(cache_base_directory)),
            internal_url_registry: self.internal_url_registry.clone(),

            #[cfg(feature = "http")]
            http_client: self.http_client.clone(),
        }
        .into()
    }
}
