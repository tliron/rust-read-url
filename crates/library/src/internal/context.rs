use super::{
    super::{context::*, errors::*, url::*},
    internal_url::*,
    registered::*,
};

use std::collections::*;

impl UrlContext {
    /// Construct an [InternalUrl].
    pub fn internal_url(
        self: &UrlContextRef,
        path_representation: String,
        host: Option<String>,
        query: Option<HashMap<String, String>>,
        fragment: Option<String>,
    ) -> UrlRef {
        InternalUrl::new(self, path_representation, false, None, host, query, fragment).into()
    }

    /// Register an [InternalUrl].
    pub fn register_internal_url(
        self: &UrlContextRef,
        path_representation: String,
        slashable: bool,
        base_path_representation: Option<String>,
        content: &'static [u8],
        format: Option<String>,
    ) -> Result<(), UrlError> {
        let mut internal_url_registry = self.internal_url_registry.lock()?;
        internal_url_registry.insert(
            path_representation,
            RegisteredInternalUrl::new(slashable, base_path_representation, content, format),
        );
        Ok(())
    }

    /// Deregister an [InternalUrl].
    pub fn deregister_internal_url(self: &UrlContextRef, path: &String) -> Result<(), UrlError> {
        let mut internal_url_registry = self.internal_url_registry.lock()?;
        internal_url_registry.remove(path);
        Ok(())
    }

    /// Register a global [InternalUrl].
    pub fn register_global_internal_url(
        path_representation: String,
        slashable: bool,
        base_path_representation: Option<String>,
        content: &'static [u8],
        format: Option<String>,
    ) -> Result<(), UrlError> {
        let mut internal_url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        internal_url_registry.insert(
            path_representation,
            RegisteredInternalUrl::new(slashable, base_path_representation, content, format),
        );
        Ok(())
    }

    /// Deregister a global [InternalUrl].
    pub fn deregister_global_internal_url(path: &String) -> Result<(), UrlError> {
        let mut internal_url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        internal_url_registry.remove(path);
        Ok(())
    }

    /// Access an [InternalUrl]'s content.
    ///
    /// Tries the context's registry first, the global registry next.
    pub fn internal_url_content(self: &UrlContextRef, path: &String) -> Result<Option<&'static [u8]>, UrlError> {
        // Try context registry first
        let internal_url_registry = self.internal_url_registry.lock()?;
        if let Some(registered_internal_url) = internal_url_registry.get(path) {
            return Ok(Some(registered_internal_url.content));
        }

        // Then the global registry
        let internal_url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        match internal_url_registry.get(path) {
            Some(registered_internal_url) => Ok(Some(registered_internal_url.content)),
            None => Ok(None),
        }
    }

    /// Access an [InternalUrl]'s metadata.
    ///
    /// Tries the context's registry first, the global registry next.
    pub fn internal_url_metadata(
        self: &UrlContextRef,
        path: &String,
    ) -> Result<Option<(bool, Option<String>, Option<String>)>, UrlError> {
        // Try context registry first
        let internal_url_registry = self.internal_url_registry.lock()?;
        if let Some(registered_internal_url) = internal_url_registry.get(path) {
            return Ok(Some(registered_internal_url.metadata()));
        }

        // Then the global registry
        let internal_url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        match internal_url_registry.get(path) {
            Some(registered_internal_url) => Ok(Some(registered_internal_url.metadata())),
            None => Ok(None),
        }
    }
}
