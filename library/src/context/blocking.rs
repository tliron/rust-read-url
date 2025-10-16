use super::{
    super::{errors::*, url::*, util::*},
    context::*,
};

use problemo::{common::*, *};

impl UrlContext {
    /// Parses the argument as either an absolute URL or a path relative to
    /// one of the context's base URls. Relative paths support ".." and ".".
    ///
    /// The returned URL will always have had [URL::conform] called on it, so
    /// there is no need to call it again.
    ///
    /// Relative paths are tested against the base URLs argument in order. The
    /// first valid URL will be returned and the remaining bases will be ignored.
    /// Note that bases can be any of any URL type.
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [url_or_file_path](UrlContext::url_or_file_path).
    pub fn url(self: &UrlContextRef, url_representation: &str) -> Result<UrlRef, Problem> {
        self.url_or_maybe_file_path(url_representation, false)
    }

    /// Parses the argument as an absolute URL, or an absolute file path, or a
    /// path relative to one of the context's base URLs. Relative paths support
    /// ".." and ".".
    ///
    /// The returned URL will always have had [URL::conform] called on it, so
    /// there is no need to call it again.
    ///
    /// Relative paths are tested against the base URLs argument in order. The
    /// first valid URL will be returned and the remaining bases will be ignored.
    /// Note that bases can be any of any URL type.
    ///
    /// On Windows note a rare edge case: If there happens to be a drive that has the
    /// same name as a supported URL scheme (e.g. "http") then callers would have to
    /// provide a full file URL, e.g. instead of "http:\Dir\file" provide
    /// "file:///http:/Dir/file". Otherwise it would be parsed as a URL of that scheme.
    /// rather than a file path.
    #[cfg(feature = "file")]
    pub fn url_or_file_path(self: &UrlContextRef, url_or_file_path_representation: &str) -> Result<UrlRef, Problem> {
        self.url_or_maybe_file_path(url_or_file_path_representation, true)
    }

    fn url_or_maybe_file_path(
        self: &UrlContextRef,
        url_or_file_path_representation: &str,
        or_file_path: bool,
    ) -> Result<UrlRef, Problem> {
        let url_or_file_path_representation = self.get_url_or_override(url_or_file_path_representation.into())?;
        match url::Url::parse(&url_or_file_path_representation) {
            Ok(url) => match url.scheme() {
                "internal" => {
                    let (query, fragment) = url_query_and_fragment(&url);
                    let mut url =
                        self.internal_url(url.path().into(), url.host_str().map(|host| host.into()), query, fragment);
                    url.conform()?;
                    Ok(url)
                }

                #[cfg(feature = "file")]
                "file" => {
                    let (query, fragment) = url_query_and_fragment(&url);
                    let mut url =
                        self.file_url(url.path().into(), url.host_str().map(|host| host.into()), query, fragment);
                    url.conform()?;
                    Ok(url)
                }

                #[cfg(feature = "http")]
                "http" | "https" => {
                    let mut url = self.http_url(url);
                    url.conform()?;
                    Ok(url)
                }

                #[cfg(feature = "tar")]
                "tar" => {
                    use super::super::tar::*;

                    let (archive_url_representation, path) = TarUrl::parse(url.as_str())?;
                    let archive_url = self.url_or_maybe_file_path(&archive_url_representation, or_file_path)?;
                    let compression = TarUrl::compression_from(&archive_url)?;
                    let mut url = self.tar_url(archive_url, path.into(), compression);
                    url.conform()?;
                    Ok(url)
                }

                #[cfg(feature = "zip")]
                "zip" => {
                    use super::super::zip::*;

                    let (repository_url_representation, path) = ZipUrl::parse(url.as_str())?;
                    let repository_url = self.url_or_maybe_file_path(&repository_url_representation, or_file_path)?;
                    let mut url = self.zip_url(repository_url, path.into());
                    url.conform()?;
                    Ok(url)
                }

                #[cfg(feature = "git")]
                "git" => {
                    use super::super::git::*;

                    let (repository_url_representation, path) = GitUrl::parse(url.as_str())?;
                    let repository_url = self.url_or_maybe_file_path(&repository_url_representation, or_file_path)?;
                    let mut url = self.git_url(repository_url, path.into())?;
                    url.conform()?;
                    Ok(url)
                }

                scheme => {
                    Err(UnsupportedError::default().into_problem().with(SchemeAttachment::new(scheme)).via(UrlError))
                }
            },

            // Not a URL
            Err(_) => {
                if or_file_path {
                    // Maybe it's an absolute file path
                    #[cfg(feature = "file")]
                    {
                        use std::path::*;

                        let path = Path::new(&url_or_file_path_representation);
                        if path.is_absolute() {
                            let mut url = self.file_url(path.into(), None, None, None);
                            url.conform()?;
                            return Ok(url);
                        }
                    }
                }

                // Try as relative
                for base_url in self.base_urls.iter() {
                    let mut url = base_url.relative(&url_or_file_path_representation);
                    if url.conform().is_ok() {
                        return Ok(url);
                    }
                }

                Err(unreachable_url(url_or_file_path_representation, "file path"))
            }
        }
    }
}
