mod utils;

use read_url::*;

pub fn main() -> Result<(), UrlError> {
    // You can override URLs in the context

    let context = UrlContext::new();

    context.register_internal_url("/hello".into(), true, None, "hello world".as_bytes(), Some("text".into()))?;

    context.override_url("http://github.com".into(), "internal:///hello".into())?;

    utils::heading("override");
    let url = context.url("http://github.com")?;
    utils::dump(&url)?;

    Ok(())
}
