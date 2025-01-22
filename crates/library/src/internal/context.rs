use super::{
    super::{context::*, errors::*, url::*},
    internal_url::*,
    metadata::*,
    registered::*,
};

use kutil_io::reader::*;

impl UrlContext {
    /// Construct an [InternalUrl].
    pub fn internal_url(
        self: &UrlContextRef,
        path: String,
        host: Option<String>,
        query: Option<UrlQuery>,
        fragment: Option<String>,
    ) -> UrlRef {
        InternalUrl::new(self, path, false, None, host, query, fragment).into()
    }

    /// Register an [InternalUrl].
    pub fn register_internal_url(
        self: &UrlContextRef,
        path: String,
        slashable: bool,
        base_path: Option<String>,
        format: Option<String>,
        content: Vec<u8>,
    ) -> Result<(), UrlError> {
        let mut url_registry = self.internal_url_registry.lock()?;
        url_registry.insert(path, RegisteredInternalUrl::new(slashable, base_path, format, content));
        Ok(())
    }

    /// Deregister an [InternalUrl].
    pub fn deregister_internal_url(self: &UrlContextRef, path: &String) -> Result<(), UrlError> {
        let mut url_registry = self.internal_url_registry.lock()?;
        url_registry.remove(path);
        Ok(())
    }

    /// Update the content of an [InternalUrl].
    pub fn update_internal_url(self: &UrlContextRef, path: &String, content: Vec<u8>) -> Result<bool, UrlError> {
        let mut url_registry = self.internal_url_registry.lock()?;
        Ok(match url_registry.get_mut(path) {
            Some(registered_internal_url) => {
                registered_internal_url.content = ReadableBuffer::new(content);
                true
            }

            None => false,
        })
    }

    /// Register a global [InternalUrl].
    pub fn register_global_internal_url(
        path: String,
        slashable: bool,
        base_path: Option<String>,
        format: Option<String>,
        content: Vec<u8>,
    ) -> Result<(), UrlError> {
        let mut url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        url_registry.insert(path, RegisteredInternalUrl::new(slashable, base_path, format, content));
        Ok(())
    }

    /// Deregister a global [InternalUrl].
    pub fn deregister_global_internal_url(path: &String) -> Result<(), UrlError> {
        let mut url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        url_registry.remove(path);
        Ok(())
    }

    /// Update the content of a global [InternalUrl].
    pub fn update_global_internal_url(path: &String, content: Vec<u8>) -> Result<bool, UrlError> {
        let mut url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        Ok(match url_registry.get_mut(path) {
            Some(registered_internal_url) => {
                registered_internal_url.content = ReadableBuffer::new(content);
                true
            }

            None => false,
        })
    }

    /// Read an [InternalUrl]'s content.
    ///
    /// Tries the context's registry first, the global registry next.
    pub fn read_internal_url(self: &UrlContextRef, path: &String) -> Result<Option<ReadableBufferReader>, UrlError> {
        // Try context registry first
        let url_registry = self.internal_url_registry.lock()?;
        if let Some(registered_internal_url) = url_registry.get(path) {
            return Ok(Some(registered_internal_url.content.reader()));
        }

        // Then the global registry
        let url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        Ok(url_registry.get(path).map(|url| url.content.reader()))
    }

    /// Access an [InternalUrl]'s metadata.
    ///
    /// Tries the context's registry first, the global registry next.
    pub fn internal_url_metadata(self: &UrlContextRef, path: &String) -> Result<Option<InternalUrlMetadata>, UrlError> {
        // Try context registry first
        let url_registry = self.internal_url_registry.lock()?;
        if let Some(registered_internal_url) = url_registry.get(path) {
            return Ok(Some(registered_internal_url.metadata.clone()));
        }

        // Then the global registry
        let url_registry = GLOBAL_INTERNAL_URL_REGISTRY.lock()?;
        Ok(url_registry.get(path).map(|url| url.metadata.clone()))
    }
}
