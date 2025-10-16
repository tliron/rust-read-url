use super::{
    super::{context::*, errors::*, url::*, util::*},
    git_url::*,
};

use {problemo::*, relative_path::*};

impl UrlContext {
    /// Construct a [GitUrl].
    pub fn git_url(
        self: &UrlContextRef,
        conformed_repository_url: UrlRef,
        path: RelativePathBuf,
    ) -> Result<UrlRef, Problem> {
        // Note: for gix we will strip the query and fragment, which is why we are also keeping the original URL
        let repository_gix_url = url_without_query_and_fragment(conformed_repository_url.to_string());
        let repository_gix_url = gix::Url::from_bytes(repository_gix_url.as_bytes().into()).into_url_problem("git")?;

        Ok(GitUrl::new(self, conformed_repository_url.into(), repository_gix_url, path).into())
    }
}
