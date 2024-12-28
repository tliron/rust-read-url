mod utils;

use read_url::*;

pub fn main() -> Result<(), UrlError> {
    // The URL fagment can be used to refer to specific versions of a git repository

    let context = UrlContext::new();

    // The default is to use the default branch tip (HEAD):

    utils::heading("default");
    let url = context.url("git:https://github.com/tliron/kutil!NOTICE")?;
    utils::dump(&url)?;

    // You can specify a branch (will use its tip):

    utils::heading("main branch");
    let url = context.url("git:https://github.com/tliron/kutil#main!NOTICE")?;
    utils::dump(&url)?;

    // Or a tag:

    utils::heading("tag");
    let url = context.url("git:https://github.com/tliron/kutil#v0.3.0!NOTICE")?;
    utils::dump(&url)?;

    // Or a commit hash:
    // (note that this will need a complete clone instead of a shallow clone)

    utils::heading("commit");
    let url = context.url("git:https://github.com/tliron/kutil#3ac4c2c2b0cb18bef8ef9bf47b6fa5baa4722c9e!NOTICE")?;
    utils::dump(&url)?;

    // Note about the cache:
    // Because read-url prefers shallow clones, it means means that all the variations above will result in separate clones
    // So the cache will *not* be shared among them

    // Final note: the fragment cannot be used with local git repositories,
    // which will always be used "as is" in their current state
    // (we wouldn't want read-url to modify them by checking out branches or commits!)

    Ok(())
}
