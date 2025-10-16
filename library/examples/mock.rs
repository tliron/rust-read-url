mod utils;

use {problemo::*, read_url::*};

pub fn main() -> Result<(), Problem> {
    // Mock URLs are useful for testing, placeholders, etc.

    let context = UrlContext::new();

    utils::heading("mock", true);
    let url = context.mock_url(
        "happy:/go=lucky".into(),
        false,                  // not "slashable"
        Some("happy:/".into()), // base URL (when "slashable" is false)
        Some(UrlQuery::from_iter([("key1".into(), "value1".into()), ("key2".into(), "value2".into())])), // query
        Some("a-fragment".into()), // fragment
        Some("text".into()),    // format
        Some(b"hello world"),   // content
    );
    utils::dump(&url)?;

    Ok(())
}
