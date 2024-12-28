mod utils;

use {read_url::*, std::collections::*};

pub fn main() -> Result<(), UrlError> {
    // Mock URLs are useful for testing, placeholders, etc.

    let context = UrlContext::new();

    utils::heading("mock");
    let url = context.mock_url(
        "happy:/go=lucky".into(),
        false,                                 // not "slashable"
        Some("happy:/".into()),                // base URL (when "slashable" is false)
        Some("hello world".as_bytes().into()), // content
        Some("text".into()),                   // format
        Some(HashMap::from([("key1".into(), "value1".into()), ("key2".into(), "value2".into())])), // query
        Some("a-fragment".into()),             // fragment
    );
    utils::dump(&url)?;

    Ok(())
}
