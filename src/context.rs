use std::{collections::*, sync::*};

//
// Context
//

pub type ContextRef = Arc<Context>;

#[derive(Debug)]
pub struct Context {
    pub files: LazyLock<HashMap<String, String>>,

    pub http_client: LazyLock<reqwest::blocking::Client>,
}

impl Context {
    pub fn new() -> ContextRef {
        Context {
            files: LazyLock::new(|| HashMap::new()),
            http_client: LazyLock::new(|| reqwest::blocking::Client::new()),
        }
        .into()
    }
}
