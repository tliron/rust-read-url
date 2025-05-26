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

mod cli;
mod errors;
mod read;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
