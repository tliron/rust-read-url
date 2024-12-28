use super::{errors::*, url::*};

use std::{collections::*, path, sync::*};

//
// Context
//

/// Common reference type for [Context].
pub type ContextRef = Arc<Context>;

/// Context for [URL](super::URL).
#[derive(Debug)]
pub struct Context {
    /// Files managed by this context.
    pub files: LazyLock<HashMap<String, String>>,

    /// Common HTTP client.
    pub http_client: LazyLock<reqwest::blocking::Client>,
}

impl Context {
    /// Constructor.
    pub fn new() -> ContextRef {
        Context {
            files: LazyLock::new(|| HashMap::new()),
            http_client: LazyLock::new(|| reqwest::blocking::Client::new()),
        }
        .into()
    }

    /// Parses the argument as an absolute URL.
    ///
    /// To support relative URLs, see [new_valid_url](Context::new_valid_url).
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [new_any_or_file_url](Context::new_any_or_file_url).
    pub fn new_url(self: &ContextRef, url: &str) -> Result<UrlRef, UrlError> {
        let url = url::Url::parse(url)?;
        match url.scheme() {
            "file" => {
                let url = self.new_file_url(path::Path::new(url.path()));
                Ok(url.into())
            }

            "http" | "https" => {
                let url = self.new_http_url(url);
                Ok(url.into())
            }

            scheme => Err(UrlError::UnsupportedScheme(scheme.into())),
        }
    }

    /// Parses the argument as either an absolute URL or an absolute file path.
    ///
    /// Internally, attempts to parse the URL via [new_url](Context::new_url) and if
    /// that fails treats the URL as a file path and returns a [FileUrl](super::FileUrl).
    ///
    /// To support relative URLs, see
    /// [new_valid_any_or_file_url](Context::new_valid_any_or_file_url).
    ///
    /// On Windows note that if there happens to be a drive that has the same
    /// name as a supported URL scheme (e.g. "http") then callers would have
    /// to provide a full file URL, e.g. instead of "http:\Dir\file" provide
    /// "file:///http:/Dir/file", otherwise it will be parsed as a URL of that
    /// scheme.
    pub fn new_any_or_file_url(self: &ContextRef, url_or_path: &str) -> Result<UrlRef, UrlError> {
        match self.clone().new_url(url_or_path) {
            Ok(url) => Ok(url),

            Err(_) => {
                let url = self.new_file_url(path::Path::new(url_or_path));
                Ok(url.into())
            }
        }
    }

    /// Parses the argument as either an absolute URL or a relative path.
    /// Relative paths support ".." and ".", with the returned URL path always
    /// being absolute.
    ///
    /// The returned URL is "valid", meaning that during this call it was
    /// possible to call [open](URL::open) on it. Of course this can't guarantee
    /// that future calls to [open](URL::open) will succeed.
    ///
    /// Relative URLs are tested against the "bases" argument in order. The
    /// first valid URL will be returned and the remaining bases will be
    /// ignored. Note that bases can be any of any URL type.
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [new_valid_any_or_file_url](Context::new_valid_any_or_file_url).
    pub fn new_valid_url(self: &ContextRef, url_or_path: &str, bases: &Vec<UrlRef>) -> Result<UrlRef, UrlError> {
        self.new_valid_url_(url_or_path, bases, false)
    }

    /// Parses the argument as an absolute URL or an absolute file path
    /// or a relative path. Relative paths support ".." and ".", with the
    /// returned URL path always being absolute.
    ///
    /// The returned URL is "valid", meaning that during this call it was
    /// possible to call [open](URL::open) on it. Of course this can't guarantee
    /// that future calls to [open](URL::open) will succeed.
    ///
    /// Relative URLs are tested against the "bases" argument in order. The
    /// first valid URL will be returned and the remaining bases will be
    /// ignored. Note that bases can be any of any URL type.
    pub fn new_valid_any_or_file_url(
        self: &ContextRef,
        url_or_path: &str,
        bases: &Vec<UrlRef>,
    ) -> Result<UrlRef, UrlError> {
        self.new_valid_url_(url_or_path, bases, true)
    }

    fn new_valid_url_(
        self: &ContextRef,
        url_or_path: &str,
        bases: &Vec<UrlRef>,
        or_file: bool,
    ) -> Result<UrlRef, UrlError> {
        match url::Url::parse(url_or_path) {
            Ok(url) => match url.scheme() {
                "file" => {
                    let url = self.new_valid_file_url(path::Path::new(url.path()))?;
                    Ok(url.into())
                }

                "http" | "https" => {
                    let url = self.new_valid_http_url(url)?;
                    Ok(url.into())
                }

                scheme => Err(UrlError::UnsupportedScheme(scheme.into())),
            },

            Err(_) => {
                if or_file {
                    let path = path::Path::new(url_or_path);
                    if path.is_absolute() {
                        let url = self.new_valid_file_url(path)?;
                        return Ok(url.into());
                    }
                }

                for base in bases {
                    if let Ok(url) = base.valid_relative(url_or_path.into()) {
                        return Ok(url);
                    }
                }

                Err(UrlError::new_not_found(url_or_path))
            }
        }
    }
}
