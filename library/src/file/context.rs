use super::{
    super::{context::*, url::*},
    file_url::*,
};

use std::path::*;

impl UrlContext {
    /// Construct a [FileUrl].
    pub fn file_url(
        self: &UrlContextRef,
        path: PathBuf,
        host: Option<String>,
        query: Option<UrlQuery>,
        fragment: Option<String>,
    ) -> UrlRef {
        FileUrl::new(self, path, host, query, fragment).into()
    }

    /// A valid [FileUrl] for the current working directory.
    #[cfg(feature = "blocking")]
    pub fn working_dir_url(self: &UrlContextRef) -> Result<UrlRef, problemo::Problem> {
        use {
            problemo::{common::*, *},
            std::env::*,
        };

        let mut url = self.file_url(current_dir().via(LowLevelError)?, None, None, None);
        url.conform()?;
        Ok(url.into())
    }

    /// Async version of [working_dir_url](UrlContext::working_dir_url).
    ///
    /// Note that the blocking version would also work in async. However, we are providing
    /// an async version, too, in case the `blocking` feature is disabled.
    #[cfg(feature = "async")]
    pub async fn working_dir_url_async(self: &UrlContextRef) -> Result<UrlRef, problemo::Problem> {
        use {
            problemo::{common::*, *},
            std::env::*,
        };

        let url = self.file_url(current_dir().via(LowLevelError)?, None, None, None);
        let url = url.conform_async()?.await?;
        Ok(url)
    }

    /// A valid [FileUrl] for the current working directory as a vector.
    ///
    /// Useful as the "base_urls" argument of [UrlContext::url].
    #[cfg(feature = "blocking")]
    pub fn working_dir_url_vec(self: &UrlContextRef) -> Result<Vec<UrlRef>, problemo::Problem> {
        Ok(vec![self.working_dir_url()?])
    }

    /// Async version of [working_dir_url_vec](UrlContext::working_dir_url_vec).
    ///
    /// Note that the blocking version would also work in async. However, we are providing
    /// an async version, too, in case the `blocking` feature is disabled.
    #[cfg(feature = "async")]
    pub async fn working_dir_url_vec_async(self: &UrlContextRef) -> Result<Vec<UrlRef>, problemo::Problem> {
        Ok(vec![self.working_dir_url_async().await?])
    }
}
