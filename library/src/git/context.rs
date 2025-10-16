use super::{
    super::{context::*, errors::*, url::*, util::*},
    errors::*,
    git_url::*,
};

use relative_path::*;

impl UrlContext {
    /// Construct a [GitUrl].
    pub fn git_url(
        self: &UrlContextRef,
        conformed_repository_url: UrlRef,
        path: RelativePathBuf,
    ) -> Result<UrlRef, UrlError> {
        // Note: for gix we will strip the query and fragment, which is why we are also keeping the original URL
        let repository_gix_url = url_without_query_and_fragment(conformed_repository_url.to_string());
        let repository_gix_url = gix::Url::from_bytes(repository_gix_url.as_bytes().into()).map_err(GitError::from)?;

        Ok(GitUrl::new(self, conformed_repository_url.into(), repository_gix_url, path).into())
    }
}
