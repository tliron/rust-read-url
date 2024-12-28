mod utils;

use read_url::*;

pub fn main() -> Result<(), UrlError> {
    // The default cache location is your operating system's temporary directory
    // (usually "/tmp")
    // But we can set it explicitly
    // Let's use "work" relative to the current work directory

    let dir = std::env::current_dir()?.join("work");
    let context = UrlContext::new_for(Some(dir));

    // The cache is necessary for two specific use cases:

    utils::heading("remote zip (download)");
    let url = context.url("zip:http://localhost:8000/archives/archive.zip!two.txt")?;
    utils::dump(&url)?;

    utils::heading("remote git (bare clone)");
    let url = context.url("git:https://github.com/tliron/rust-read-url.git!assets/files/two.txt")?;
    utils::dump(&url)?;

    // Note that remote "tar:" URLs do not use the cache!
    // That's because tar is a streaming format
    // So if you need to accesss remote archives, you might want to prefer tarballs over zips

    // Subsequent references to files in the same archive/repository will use the existing cache:

    utils::heading("remote zip (use cache)");
    let url = context.url("zip:http://localhost:8000/archives/archive.zip!three.txt")?;
    utils::dump(&url)?;

    utils::heading("remote git (use cache)");
    let url = context.url("git:https://github.com/tliron/rust-read-url.git!assets/files/three.txt")?;
    utils::dump(&url)?;

    // Important notes about the cache:
    // * The cache is thread-safe
    // * Created files will be deleted upon the context's drop()
    //   But some kinds of crashes may cause drop() to not be called

    // Note that the cache is only used when the source is *remote*
    // The following examples do *not* use the cache

    let context = context.with_base_urls(context.working_dir_url_vec()?);

    utils::heading("local zip");
    let url = context.url("zip:assets/archives/archive.zip!two.txt")?;
    utils::dump(&url)?;

    // (Note that below we are assuming you are running in a local git clone)

    utils::heading("local git");
    let url = context.url("git:!assets/files/two.txt")?;
    utils::dump(&url)?;

    Ok(())
}
