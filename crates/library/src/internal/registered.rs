use std::{collections::*, sync::*};

//
// RegisteredInternalUrl
//

/// Global [InternalUrl](super::internal_url::InternalUrl) registry.
pub static GLOBAL_INTERNAL_URL_REGISTRY: LazyLock<InternalUrlRegistry> = LazyLock::new(|| HashMap::new().into());

/// [InternalUrl](super::internal_url::InternalUrl) registry.
pub type InternalUrlRegistry = Mutex<HashMap<String, RegisteredInternalUrl>>;

/// Registered [InternalUrl](super::internal_url::InternalUrl).
#[derive(Debug)]
pub struct RegisteredInternalUrl {
    /// Whether the path representation is "slashable".
    pub slashable: bool,

    /// The optional base path representation (used when slashable is false).
    pub base_path_representation: Option<String>,

    /// Content.
    pub content: &'static [u8],

    /// Format.
    pub format: Option<String>,
}

impl RegisteredInternalUrl {
    /// Constructor
    pub fn new(
        slashable: bool,
        base_path_representation: Option<String>,
        content: &'static [u8],
        format: Option<String>,
    ) -> Self {
        Self { slashable, base_path_representation, content, format }
    }

    /// Metadata.
    pub fn metadata(&self) -> (bool, Option<String>, Option<String>) {
        (self.slashable, self.base_path_representation.clone(), self.format.clone())
    }
}
