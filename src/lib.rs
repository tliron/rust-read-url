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
mod file;
mod http;
mod url;

#[allow(unused_imports)]
pub use {context::*, errors::*, file::*, http::*, url::*};
