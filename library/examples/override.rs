mod utils;

use {problemo::*, read_url::*};

pub fn main() -> Result<(), Problem> {
    // You can override URLs in the context

    let context = UrlContext::new();

    context.register_internal_url("/hello".into(), true, None, Some("text".into()), b"hello world")?;

    context.override_url("http://github.com".into(), "internal:///hello".into())?;

    utils::heading("override", true);
    let url = context.url("http://github.com")?;
    utils::dump(&url)?;

    Ok(())
}
