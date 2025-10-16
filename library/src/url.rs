use super::context::*;

use {
    kutil::std::collections::*,
    problemo::*,
    std::{fmt, io, path::*},
};

#[cfg(feature = "async")]
use {
    std::{future::*, pin::*},
    tokio::io::AsyncRead,
};

/// Common reference type for [URL].
pub type UrlRef = Box<dyn URL + Send + Sync>;

/// URL query.
pub type UrlQuery = FastHashMap<String, String>;

/// Common reference type for [Read](io::Read).
pub type ReadRef = Box<dyn io::Read + Send + Sync>;

/// Common reference type for [AsyncRead].
#[cfg(feature = "async")]
pub type AsyncReadRef = Pin<Box<dyn AsyncRead>>;

/// Common [Future] type for [URL::conform_async].
#[cfg(feature = "async")]
pub type ConformFuture = Pin<Box<dyn Future<Output = Result<UrlRef, Problem>>>>;

/// Common [Future] type for [URL::open_async].
#[cfg(feature = "async")]
pub type OpenFuture = Pin<Box<dyn Future<Output = Result<AsyncReadRef, Problem>>>>;

/// Format an archive URL.
pub fn format_archive_url(scheme: &str, archive: &str, path: &str) -> String {
    format!("{}:{}!{}", scheme, archive.replace('!', "%21"), path)
}

//
// URL
//

/// URL.
pub trait URL
where
    Self: fmt::Debug + fmt::Display,
{
    /// The [UrlContext] used to create this URL.
    fn context(&self) -> &UrlContext;

    /// Clone as reference.
    fn cloned(&self) -> UrlRef;

    /// Returns a string that uniquely identifies the URL.
    ///
    /// Useful as a map or cache key.
    fn key(&self) -> String {
        format!("{}", self)
    }

    /// The optional query.
    fn query(&self) -> Option<UrlQuery> {
        None
    }

    /// The optional fragment.
    fn fragment(&self) -> Option<String> {
        None
    }

    /// Format of the URL content's canonical representation.
    ///
    /// Can return "text", "yaml", "json", "tar", "tar.gz", etc.
    ///
    /// The format is often derived from a file extension if available, otherwise
    /// it might be retrieved from metadata.
    ///
    /// An attempt is made to standardize the return values, e.g. a "yml" file
    /// extension is always returned as "yaml", and a "tgz" file extension is
    /// always returned as "tar.gz".
    fn format(&self) -> Option<String> {
        None
    }

    /// If this URL points to a local path, returns it.
    fn local(&self) -> Option<PathBuf> {
        None
    }

    /// Returns a URL that is the equivalent of a "base directory" for the URL.
    ///
    /// The base URL will normally *not* have the query and fragment of this URL.
    ///
    /// Note that the base might not be readable, e.g. you would not be able to call
    /// [open](URL::open) on it if it is a filesystem directory.
    fn base(&self) -> Option<UrlRef> {
        None
    }

    /// Parses the argument as a path relative to the URL. That means that this
    /// URL is treated as a "base directory" (see [base](URL::base)). The argument
    /// supports ".." and ".", with the returned URL path always being absolute.
    ///
    /// The relative URL will normally *not* have the query and fragment of this URL.
    fn relative(&self, path: &str) -> UrlRef;

    /// Ensures that the URL conforms with the expectations of its functions. If
    /// successful, this function may change the URL appropriately, e.g. a relative
    /// path would be turned into an absolute path.
    ///
    /// This includes the expectation that [open](URL::open) would minimally succeed,
    /// e.g. that the file exists or that the network endpoint is responsive. It does
    /// not otherwise guarantee that reading would be successful.
    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), Problem>;

    /// Async version of [URL::conform].
    ///
    /// Am important difference is that instead of mutating the URL it returns the
    /// new conformed version.
    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, Problem>;

    /// Opens the URL for reading by providing a `dyn` [Read][io::Read].
    ///
    /// Note that for some URLs it may involve lengthy operations, e.g. cloning a
    /// remote repository, download a file, and/or unpacking an archive.
    ///
    /// Thus, an effort is made to not repeat these lengthy operations by caching
    /// relevant state via the URL's [UrlContext]. For example, when accessing a "git:"
    /// URL on a remote repository that repository will be cloned locally only if it's
    /// the first time the repository has been referred to for the [UrlContext].
    /// Subsequent [open](URL::open) calls for URLs that refer to the same git
    /// repository will reuse the existing clone.
    ///
    /// An effect of this optimization is that you might not be reading the most
    /// recent version of the resource the URL points to. If that is undesirable,
    /// call [reset](super::cache::UrlCache::reset) on the [UrlContext] cache.
    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, Problem>;

    /// Async version of [URL::open]. Provides a `dyn` [AsyncRead](tokio::io::AsyncRead).
    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, Problem>;
}
