mod utils;

use {read_url::*, tokio::*};

// To use async make sure to enable the "async" crate feature in your Cargo.toml

#[main]
pub async fn main() -> Result<(), UrlError> {
    // We'll show some of the same stuff as in examples/start_here.rs,
    // but in async (via tokio)
    // All relevant APIs have _async equivalents

    let context = UrlContext::new();

    utils::heading("http", true);
    let url = context.url_async("http://localhost:8000/files/two.txt").await?;
    utils::dump_async(&url).await?;

    utils::heading("https", false);
    let url = context
        .url_async("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/main/assets/files/two.txt")
        .await?;
    utils::dump_async(&url).await?;

    utils::heading("tar (no compression)", false);
    let url = context.url_async("tar:http://localhost:8000/archives/archive.tar!two.txt").await?;
    utils::dump_async(&url).await?;

    utils::heading("tar (gzip)", false);
    let url = context.url_async("tar:http://localhost:8000/archives/archive.tar.gz!two.txt").await?;
    utils::dump_async(&url).await?;

    utils::heading("tar (zstd)", false);
    let url = context.url_async("tar:http://localhost:8000/archives/archive.tar.zst!two.txt").await?;
    utils::dump_async(&url).await?;

    utils::heading("zip", false);
    let url = context.url_async("zip:http://localhost:8000/archives/archive.zip!two.txt").await?;
    utils::dump_async(&url).await?;

    utils::heading("git", false);
    let url = context.url_async("git:https://github.com/tliron/rust-read-url.git!assets/files/two.txt").await?;
    utils::dump_async(&url).await?;

    // Note that absolute does need an async version
    // But that conform_async() works a bit differently from conform(),
    // in that you need to use its returned value

    utils::heading("absolute", false);
    let mut url = context
        .absolute_url("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/main/assets/files/two.txt")?;
    url = url.conform_async()?.await?;
    utils::dump_async(&url).await?;

    Ok(())
}
