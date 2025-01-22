use super::metadata::*;

use {
    kutil_io::reader::*,
    std::{collections::*, sync::*},
};

//
// RegisteredInternalUrl
//

/// Global [InternalUrl](super::internal_url::InternalUrl) registry.
pub static GLOBAL_INTERNAL_URL_REGISTRY: LazyLock<InternalUrlRegistry> =
    LazyLock::new(|| RegisteredInternalUrls::new().into());

/// [InternalUrl](super::internal_url::InternalUrl) registry.
pub type InternalUrlRegistry = Mutex<RegisteredInternalUrls>;

/// [InternalUrl](super::internal_url::InternalUrl) map.
pub type RegisteredInternalUrls = HashMap<String, RegisteredInternalUrl>;

/// Registered [InternalUrl](super::internal_url::InternalUrl).
#[derive(Debug)]
pub struct RegisteredInternalUrl {
    /// Metadata.
    pub metadata: InternalUrlMetadata,

    /// Content.
    pub content: ReadableBuffer,
}

impl RegisteredInternalUrl {
    /// Constructor
    pub fn new(slashable: bool, base_path: Option<String>, format: Option<String>, content: Vec<u8>) -> Self {
        Self { metadata: InternalUrlMetadata::new(slashable, base_path, format), content: ReadableBuffer::new(content) }
    }
}
