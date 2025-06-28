use super::super::{context::*, errors::*, url::*, util::*};

use {
    relative_path::*,
    std::{fmt, sync::*},
};

//
// GitUrl
//

/// A URL for an entry in a git repository.
///
/// The URL scheme is "git:", followed by repository URL, a `!`, and then the entry path
/// within the repository. The fragment of the repository URL is used to select a git tag,
/// a commit hash in hex, or a branch name (in which case the tip of the branch will be used).
/// The default is to use the tip of the default branch.
///
/// Note that the fragment cannot be used with local repositories, which will be accessed in
/// their current state.
#[derive(Clone, Debug)]
pub struct GitUrl {
    /// The repository [URL].
    pub repository_url: Arc<UrlRef>,

    /// The repository [gix Url](gix::Url).
    pub repository_gix_url: gix::Url,

    /// The entry path.
    pub path: RelativePathBuf,

    pub(crate) context: UrlContextRef,
}

impl GitUrl {
    /// Parse.
    pub fn parse(url_representation: &str) -> Result<(String, String), UrlError> {
        parse_archive_entry_url_representation(url_representation, "git")
    }

    /// Constructor.
    pub fn new(
        context: &UrlContextRef,
        repository_url: Arc<UrlRef>,
        repository_gix_url: gix::Url,
        path: RelativePathBuf,
    ) -> Self {
        Self { repository_url, repository_gix_url, path, context: context.clone() }
    }

    /// Constructor.
    pub fn new_with(&self, path: RelativePathBuf) -> Self {
        Self::new(&self.context, self.repository_url.clone(), self.repository_gix_url.clone(), path)
    }
}

impl fmt::Display for GitUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "git:{}!{}", self.repository_url, self.path)
    }
}

// Conversions

impl Into<UrlRef> for GitUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
