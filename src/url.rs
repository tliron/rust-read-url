use super::{context::*, errors::*};

use std::{fmt::Display, io::Read, path::Path};

//
// URL
//

/// Common reference type for [URL].
pub type UrlRef = Box<dyn URL>;

/// URL.
pub trait URL: Display {
    /// The [Context] used to create this URL.
    fn context(&self) -> &Context;

    /// Format of the URL content's canonical representation.
    ///
    /// Can return "yaml", "json", "xml", etc., or an empty string if the format
    /// is unknown.
    ///
    /// The format is often derived from a file extension if available, otherwise
    /// it might be retrieved from metadata.
    ///
    /// An attempt is made to standardize the return values, e.g. a "yml" file
    /// extension is always returned as "yaml", and a "tar.gz" file extension is
    /// always returned as "tgz".
    fn format(&self) -> Option<String>;

    /// Returns a URL that is the equivalent of a "base directory" for this URL.
    ///
    /// Base URLs always have a trailing slash to signify that they are
    /// "directories" rather than "files". One notable exception is "file:" URLs
    /// when compiled on Windows, in which case a trailing backslash is used
    /// instead.
    ///
    /// The base is often used in two ways:
    ///
    /// 1. You can call [relative](URL::relative) on it to get a sibling URL to this
    ///    one (relative to the same "base directory").
    /// 2. You can use it in the "bases" list argument of [Context::new_valid_url] for
    ///    the same purpose.
    ///
    /// Note that the base might not be a valid URL in itself, e.g. you might not
    /// be able to call [open](URL::open) on it.
    fn base(&self) -> Option<UrlRef>;

    /// Parses the argument as a path relative to the URL. That means that this
    /// URL is treated as a "base directory" (see [base](URL::base). The argument
    /// supports ".." and ".", with the returned URL path always being absolute.
    fn relative(&self, path: &str) -> UrlRef;

    /// As [relative](URL::relative) but returns a valid URL.
    fn valid_relative(&self, path: &str) -> Result<UrlRef, UrlError>;

    /// Returns a string that uniquely identifies the URL.
    ///
    /// Useful for map and cache keys.
    fn key(&self) -> String;

    /// Opens the URL for reading.
    ///
    /// Note that for some URLs it can involve lengthy operations, e.g. cloning a
    /// remote repository or downloading an archive. TODO: how to cancel?
    ///
    /// An effort is made to not repeat these lengthy operations by caching related
    /// state in the URL's [Context] (caching is deliberately not done globally).
    /// For example, when accessing a "git:" URL on a remote git repository then that
    /// repository will be cloned locally only if it's the first the repository has been
    /// referred to for the exturl Context. Subsequent [open](URL::open) calls for URLs
    /// that refer to the same git repository will reuse the existing clone.
    fn open(&self) -> Result<Box<dyn Read>, UrlError>;
}

impl Context {
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
                let url = self.new_file_url(Path::new(url.path()));
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
                let url = self.new_file_url(Path::new(url_or_path));
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
                    let url = self.new_valid_file_url(Path::new(url.path()))?;
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
                    let path = Path::new(url_or_path);
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
