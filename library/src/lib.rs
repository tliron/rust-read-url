// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Go beyond `http://` with this streamlined URL library for Rust.

read-url gets you an [io::Read](std::io::Read) or a [tokio::io::AsyncRead] from a wide variety of URL types,
including entries in archives and code repositories using a URL notation inspired by Java's
[JarURLConnection](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/net/JarURLConnection.html).

For more information and usage examples see the
[home page](https://github.com/tliron/rust-read-url).
*/

mod cache;
mod context;
mod errors;
mod url;
mod util;

/// `file:`
#[cfg(feature = "file")]
pub mod file;

/// `git:`
#[cfg(feature = "git")]
pub mod git;

/// `http:` and `https:`
#[cfg(feature = "http")]
pub mod http;

/// `internal:`
pub mod internal;

/// Mock URLs.
pub mod mock;

/// `tar:`
#[cfg(feature = "tar")]
pub mod tar;

/// `zip:`
#[cfg(feature = "zip")]
pub mod zip;

#[allow(unused_imports)]
pub use {cache::*, context::*, errors::*, url::*};
