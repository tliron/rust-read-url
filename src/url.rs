use super::{context::*, errors::*};

use std::{fmt, io};

//
// URL
//

/// Common reference type for [URL].
pub type UrlRef = Box<dyn URL>;

/// URL.
pub trait URL: fmt::Display {
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
    fn open(&self) -> Result<Box<dyn io::Read>, UrlError>;
}
