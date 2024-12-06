use std::{collections::*, sync::*};

//
// Context
//

/// Common reference type for [Context].
pub type ContextRef = Arc<Context>;

/// Context for [URL](super::URL).
#[derive(Debug)]
pub struct Context {
    /// Files managed by this context.
    pub files: LazyLock<HashMap<String, String>>,

    /// Common HTTP client.
    pub http_client: LazyLock<reqwest::blocking::Client>,
}

impl Context {
    /// Constructor.
    pub fn new() -> ContextRef {
        Context {
            files: LazyLock::new(|| HashMap::new()),
            http_client: LazyLock::new(|| reqwest::blocking::Client::new()),
        }
        .into()
    }
}
