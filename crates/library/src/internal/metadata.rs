//
// InternalUrlMetadata
//

/// Internal URL metadata.
#[derive(Clone, Debug)]
pub struct InternalUrlMetadata {
    /// Whether the path representation is "slashable".
    pub slashable: bool,

    /// The optional base path (used when slashable is false).
    pub base_path: Option<String>,

    /// The optional format.
    pub format: Option<String>,
}

impl InternalUrlMetadata {
    /// Constructor
    pub fn new(slashable: bool, base_path: Option<String>, format: Option<String>) -> Self {
        Self { slashable, base_path, format }
    }
}
