use super::{
    super::{errors::*, url::*, util::*},
    context::*,
};

use problemo::{common::*, *};

impl UrlContext {
    /// Parses the argument as an absolute URL.
    ///
    /// Make sure to call `URL::conform` or `URL::conform_async` before calling
    /// `URL::open` or `URL::open_async`.
    ///
    /// To support relative URLs, see [url](UrlContext::url).
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [absolute_url_or_file_path](UrlContext::absolute_url_or_file_path).
    pub fn absolute_url(self: &UrlContextRef, url_representation: &str) -> Result<UrlRef, Problem> {
        let url_representation = self.get_url_or_override(url_representation.into())?;

        let url = url::Url::parse(&url_representation)
            .via(MalformedError::default())
            .with(UrlAttachment::new(url_representation))
            .via(UrlError)?;

        match url.scheme() {
            "internal" => {
                let (query, fragment) = url_query_and_fragment(&url);
                Ok(self.internal_url(url.path().into(), url.host_str().map(|host| host.into()), query, fragment))
            }

            #[cfg(feature = "file")]
            "file" => {
                let (query, fragment) = url_query_and_fragment(&url);
                Ok(self.file_url(url.path().into(), url.host_str().map(|host| host.into()), query, fragment))
            }

            #[cfg(feature = "http")]
            "http" | "https" => Ok(self.http_url(url)),

            #[cfg(feature = "tar")]
            "tar" => {
                use super::super::tar::*;

                let (archive_url_representation, path) = TarUrl::parse(url.as_str())?;
                let archive_url = self.absolute_url(&archive_url_representation)?;
                let compression = TarUrl::compression_from(&archive_url)?;
                Ok(self.tar_url(archive_url, path.into(), compression))
            }

            #[cfg(feature = "zip")]
            "zip" => {
                use super::super::zip::*;

                let (archive_url_representation, path) = ZipUrl::parse(url.as_str())?;
                let archive_url = self.absolute_url(&archive_url_representation)?;
                Ok(self.zip_url(archive_url, path.into()))
            }

            #[cfg(feature = "git")]
            "git" => {
                use super::super::git::*;

                let (repository_url_representation, path) = GitUrl::parse(url.as_str())?;
                let repository_url = self.absolute_url(&repository_url_representation)?;
                Ok(self.git_url(repository_url, path.into())?)
            }

            scheme => Err(UnsupportedError::as_problem("URL scheme").with(SchemeAttachment::new(scheme)).via(UrlError)),
        }
    }

    /// Parses the argument as either an absolute URL or an absolute file path.
    ///
    /// Make sure to call `URL::conform` or `URL::conform_async` before calling
    /// `URL::open` or `URL::open_async`.
    ///
    /// Internally, attempts to parse the URL via
    /// [absolute_url](super::super::UrlContext::absolute_url) and if that fails treats
    /// the URL as an absolute file path and returns a
    /// [FileUrl](super::super::file::FileUrl).
    ///
    /// To support relative URLs and relative file paths, see
    /// [url_or_file_path](UrlContext::url_or_file_path).
    ///
    /// On Windows note a rare edge case: If there happens to be a drive that has the
    /// same name as a supported URL scheme (e.g. "http") then callers would have to
    /// provide a full file URL, e.g. instead of "http:\Dir\file" provide
    /// "file:///http:/Dir/file". Otherwise it would be parsed as a URL of that scheme.
    /// rather than a file path.
    #[cfg(feature = "file")]
    pub fn absolute_url_or_file_path(
        self: &UrlContextRef,
        url_or_file_path_representation: &str,
    ) -> Result<UrlRef, Problem> {
        match self.clone().absolute_url(url_or_file_path_representation) {
            Ok(url) => Ok(url),

            Err(_) => Ok(self.file_url(url_or_file_path_representation.into(), None, None, None)),
        }
    }
}
