mod utils;

use read_url::*;

pub fn main() -> Result<(), UrlError> {
    // The URL fagment can be used to refer to specific versions of a git repository

    let context = UrlContext::new();

    // The default is to use the default branch tip (HEAD):

    utils::heading("default", true);
    let url = context.url("git:https://github.com/tliron/kutil!LICENSE-MIT")?;
    utils::dump(&url)?;

    // You can specify a branch (will use its tip):

    utils::heading("main branch", false);
    let url = context.url("git:https://github.com/tliron/kutil#main!LICENSE-MIT")?;
    utils::dump(&url)?;

    // Or a tag:

    utils::heading("tag", false);
    let url = context.url("git:https://github.com/tliron/kutil#r1!LICENSE-MIT")?;
    utils::dump(&url)?;

    // Or a commit hash:
    // (note that this will need a complete clone instead of a shallow clone)

    utils::heading("commit", false);
    let url =
        context.url("git:https://github.com/tliron/kutil#499b56af97b3fdbab0bac75c764c46ad623e2937!LICENSE-MIT")?;
    utils::dump(&url)?;

    // Note about the cache:
    // Because read-url prefers shallow clones, it means means that all the variations above will result in separate clones
    // So the cache will *not* be shared among them

    // Final note: the fragment cannot be used with local git repositories,
    // which will always be used "as is" in their current state
    // (we wouldn't want read-url to modify them by checking out branches or commits!)

    Ok(())
}
