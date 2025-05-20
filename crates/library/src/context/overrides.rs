use super::{super::errors::*, context::*};

use std::{collections::*, sync::*};

/// Global URL overrides.
pub static GLOBAL_URL_OVERRIDES: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| HashMap::new().into());

impl UrlContext {
    /// Override a URL.
    pub fn override_url(self: &UrlContextRef, from_url: String, to_url: String) -> Result<Option<String>, UrlError> {
        let mut url_overrides = self.url_overrides.lock()?;
        Ok(url_overrides.insert(from_url, to_url))
    }

    /// Remove a URL override.
    pub fn remove_url_override(self: &UrlContextRef, from_url: &String) -> Result<Option<String>, UrlError> {
        let mut url_overrides = self.url_overrides.lock()?;
        Ok(url_overrides.remove(from_url))
    }

    /// Override a global URL.
    pub fn override_global_url(from_url: String, to_url: String) -> Result<Option<String>, UrlError> {
        let mut url_overrides = GLOBAL_URL_OVERRIDES.lock()?;
        Ok(url_overrides.insert(from_url, to_url))
    }

    /// Remove a global URL override.
    pub fn remove_global_url_override(from_url: &String) -> Result<Option<String>, UrlError> {
        let mut url_overrides = GLOBAL_URL_OVERRIDES.lock()?;
        Ok(url_overrides.remove(from_url))
    }

    /// Get a URL override.
    ///
    /// Tries the context's overrides first, the global overrides next.
    pub fn get_url_override(self: &UrlContextRef, from_url: &String) -> Result<Option<String>, UrlError> {
        // Try context overrides first
        let url_overrides = self.url_overrides.lock()?;
        if let Some(to_url) = url_overrides.get(from_url) {
            return Ok(Some(to_url.clone()));
        }

        // Then the global overrides
        let url_overrides = GLOBAL_URL_OVERRIDES.lock()?;
        Ok(url_overrides.get(from_url).cloned())
    }

    /// Get a URL's override or itself.
    ///
    /// Tries the context's overrides first, the global overrides next.
    pub fn get_url_or_override(self: &UrlContextRef, from_url: String) -> Result<String, UrlError> {
        Ok(self.get_url_override(&from_url)?.clone().unwrap_or(from_url))
    }
}
