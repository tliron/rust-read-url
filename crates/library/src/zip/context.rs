use super::{
    super::{context::*, url::*},
    zip_url::*,
};

use {relative_path::*, std::sync::*};

impl UrlContext {
    /// Construct a [ZipUrl].
    pub fn zip_url(self: &UrlContextRef, conformed_archive_url: UrlRef, path: RelativePathBuf) -> UrlRef {
        ZipUrl::new(self, Arc::new(conformed_archive_url), path).into()
    }
}
