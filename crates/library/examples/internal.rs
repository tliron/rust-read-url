mod utils;

use read_url::*;

pub fn main() -> Result<(), UrlError> {
    // Internal URLs are useful for providing custom data to consumers of the read-url API

    UrlContext::register_global_internal_url(
        "/my/content".into(),
        true,                                   // "slashable"
        None,                                   // base URL (when "slashable" is false)
        Some("text".into()),                    // format
        "global hello world".as_bytes().into(), // content
    )?;

    let context = UrlContext::new();

    utils::heading("internal (global)");
    let url = context.url("internal:///my/content")?;
    utils::dump(&url)?;

    // The context registry will override the global registry

    context.register_internal_url(
        "/my/content".into(),
        true,
        None,
        Some("text".into()),
        "context hello world".as_bytes().into(),
    )?;

    utils::heading("internal (context)");
    let url = context.url("internal:///my/content")?;
    utils::dump(&url)?;

    // Note that internal URLs actually being with two slashes, not three
    // The host is in there, too, and read-url preserves it for representation purposes
    // (this is the standard URL format)
    // You can use query and fragment, too

    utils::heading("internal (host and query and fragment)");
    let url = context.url("internal://host/my/content?key1=value1&key2=value2#extra-stuff-here")?;
    utils::dump(&url)?;

    // You can use internal URLs as base URLs:

    let context = context.with_base_urls(vec![context.absolute_url("internal:///my/")?]);

    utils::heading("internal (relative)");
    let url = context.url("content")?;
    utils::dump(&url)?;

    Ok(())
}
