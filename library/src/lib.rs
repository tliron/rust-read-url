// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

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
