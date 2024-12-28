use thiserror::*;

//
// GitError
//

/// Git error.
#[derive(Error, Debug)]
pub enum GitError {
    /// Parse.
    #[error("parse: {0}")]
    Parse(#[from] gix::url::parse::Error),

    /// Open.
    #[error("open: {0}")]
    Open(#[from] gix::open::Error),

    /// Clone.
    #[error("clone: {0}")]
    Clone(#[from] gix::clone::Error),

    /// Fetch.
    #[error("fetch: {0}")]
    Fetch(#[from] gix::clone::fetch::Error),

    /// Head tree.
    #[error("head tree: {0}")]
    HeadTree(#[from] gix::reference::head_tree::Error),

    /// Reference.
    #[error("reference: {0}")]
    Reference(#[from] gix::refs::name::Error),

    /// Decode.
    #[error("decode: {0}")]
    Decode(#[from] gix::diff::object::decode::Error),

    /// Find.
    #[error("find: {0}")]
    Find(#[from] gix::object::find::existing::Error),

    /// Find with conversion.
    #[error("find with conversion: {0}")]
    FindWithConversion(#[from] gix::object::find::existing::with_conversion::Error),

    /// Commit object.
    #[error("commit object: {0}")]
    CommitObject(#[from] gix::object::commit::Error),

    /// Into.
    #[error("into: {0}")]
    Into(#[from] gix::object::try_into::Error),
}
