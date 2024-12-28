use super::{
    super::{context::*, errors::*, url::*},
    errors::*,
    git_url::*,
};

use {relative_path::*, std::sync::*};

impl UrlContext {
    /// Construct a [GitUrl].
    pub fn git_url(
        self: &UrlContextRef,
        conformed_repository_url: UrlRef,
        path: RelativePathBuf,
    ) -> Result<UrlRef, UrlError> {
        // Note: gix will strip the query and fragment, which is why we are also keeping the original URL
        let repository_gix_url = gix::Url::from_bytes(conformed_repository_url.to_string().as_bytes().into())
            .map_err(|e| GitError::from(e))?;

        Ok(GitUrl::new(self, Arc::new(conformed_repository_url), repository_gix_url, path).into())
    }
}
