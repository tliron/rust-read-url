// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Go beyond `http://` with this advanced URL library for Rust.

read-url gets you a [std::io::Read] from a wide variety of URL types, including
specific entries in archives using a URL structure inspired by Java's
[JarURLConnection](https://docs.oracle.com/javase/8/docs/api/java/net/JarURLConnection.html).

For more information and usage examples see the
[home page](https://github.com/tliron/rust-read-url).
*/

mod context;
mod errors;
#[cfg(feature = "file")]
mod file;
#[cfg(feature = "http")]
mod http;
mod url;

#[allow(unused_imports)]
pub use {context::*, errors::*, url::*};

#[cfg(feature = "file")]
pub use file::*;

#[cfg(feature = "http")]
pub use http::*;
