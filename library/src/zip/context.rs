use super::{
    super::{context::*, url::*},
    zip_url::*,
};

use relative_path::*;

impl UrlContext {
    /// Construct a [ZipUrl].
    pub fn zip_url(self: &UrlContextRef, conformed_archive_url: UrlRef, path: RelativePathBuf) -> UrlRef {
        ZipUrl::new(self, conformed_archive_url.into(), path).into()
    }
}
