mod utils;

use read_url::*;

// Running this example assumes that our working dir is the repository clone's root

// It also assumes a local HTTP server at port 8000 serving the files in the /assets directory
// A quick way to get one up and running:
//
//     cd assets
//     python -m http.server

// Also note that when adding read-url to your Cargo.toml, you *must* enable a crate feature to
// select an API, either "blocking" or "async"
//
// Here we use "blocking"

pub fn main() -> Result<(), UrlError> {
    // Let's demonstrate the read-url APIs

    // You always need a context:

    let context = UrlContext::new();

    // The context has a cache, base URLs, an internal URL registry, and keeps track of URL overrides
    // So it's likely that you'd want to re-use the same context in various parts of your program
    // For this reason the "new" function above returns an Arc,
    // Making it easy to clone it and move it around as necessary
    // It's thread-safe, too

    // The simplest way to get a URL is the url() API
    // Look at all these URL types!
    // (Also look at the code in examples/utils/ to see how we access the URLs)

    utils::heading("http");
    let url = context.url("http://localhost:8000/files/two.txt")?;
    utils::dump(&url)?;

    utils::heading("https");
    let url =
        context.url("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/main/assets/files/two.txt")?;
    utils::dump(&url)?;

    utils::heading("tar (no compression)");
    let url = context.url("tar:http://localhost:8000/archives/archive.tar!two.txt")?;
    utils::dump(&url)?;

    utils::heading("tar (gzip)");
    let url = context.url("tar:http://localhost:8000/archives/archive.tar.gz!two.txt")?;
    utils::dump(&url)?;

    utils::heading("tar (zstd)");
    let url = context.url("tar:http://localhost:8000/archives/archive.tar.zst!two.txt")?;
    utils::dump(&url)?;

    utils::heading("tar (nested)");
    let url = context.url("tar:tar:http://localhost:8000/archives/nested.tar.gz!archive.tar.gz!two.txt")?;
    utils::dump(&url)?;

    utils::heading("zip");
    let url = context.url("zip:http://localhost:8000/archives/archive.zip!two.txt")?;
    utils::dump(&url)?;

    utils::heading("git");
    let url = context.url("git:https://github.com/tliron/rust-read-url.git!assets/files/two.txt")?;
    utils::dump(&url)?;

    // The url() API also supports relative URLs, but relative to what?
    // We need to provide the context with "base URLs":

    let context = context.with_base_urls(vec![
        context.working_dir_url()?,
        context.absolute_url("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/")?,
    ]);

    // Notes:
    // * The call above created a "child context" that shares everything with the parent context,
    //   except the base URLs in this case
    // * It's cheap to create child contexts
    // * They do not refer to the parent (it's all Arcs and Mutexes internally),
    //   so they can be moved (and cloned)
    // * We've explicitly added the current working directory
    //   it's not there by default for reasons of security
    // * The base URLs are checked against in order
    // * Base URLs must have a trailing slash to ensure proper path joining
    //   (the URL.base() API always adds a trailing slash when necessary)

    // Now we can provide url() with a relative path instead of a full URL
    // Note that the returned URL will be full (with scheme) and absolute

    utils::heading("relative to working dir");
    let url = context.url("assets/files/two.txt")?;
    utils::dump(&url)?;

    utils::heading("relative to http");
    let url = context.url("tar:assets/archives/archive.tar.gz!two.txt")?;
    utils::dump(&url)?;

    // When we created the base URLs above, we actually used absolute_url() instead of url()
    // It's like url() but
    // * only accepts absolute URLs (duh)
    // * does not call conform() on the URL, so it's possible to give it URLs that do not have endpoints;
    //   In our case the URL would return a 404 Not Found, so indeed we *had* to use absolute_url()
    // * you should call conform() manually on it before trying to open() it
    //   like so:

    utils::heading("absolute");
    let mut url = context
        .absolute_url("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/main/assets/files/two.txt")?;
    url.conform()?;
    utils::dump(&url)?;

    // It's common to want to accept *either* a URL *or* a local file path as input
    // That's what the url_or_file_path() API is for:

    utils::heading("url_or_file_path (absolute file)");
    let url = context.url_or_file_path("/etc/fstab")?;
    utils::about(&url);

    utils::heading("url_or_file_path (absolute url)");
    let url = context.url_or_file_path("http://localhost:8000/files/two.txt")?;
    utils::dump(&url)?;

    utils::heading("url_or_file_path (relative)");
    let url = context.url_or_file_path("assets/files/two.txt")?;
    utils::dump(&url)?;

    // By the way, files can be provided as either an absolute or relative path
    // (which is operating-system dependent; very different on Windows!)
    // or a more universal (absolute) "file:" URL:

    utils::heading("file");
    let url = context.url_or_file_path("file:///etc/fstab")?;
    utils::about(&url);

    // Finally, let's just show that read-url preserves the URL query and fragment:

    utils::heading("query and fragment");
    let url = context.url_or_file_path("file:///etc/fstab?key1=value1&key2=value2#extra-stuff-here")?;
    utils::about(&url);

    Ok(())
}
