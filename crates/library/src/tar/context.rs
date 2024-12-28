use super::{
    super::{context::*, url::*},
    compression::*,
    tar_url::*,
};

use {relative_path::*, std::sync::*};

impl UrlContext {
    /// Construct a [TarUrl].
    pub fn tar_url(
        self: &UrlContextRef,
        conformed_archive_url: UrlRef,
        path: RelativePathBuf,
        compression: Option<TarCompression>,
    ) -> UrlRef {
        TarUrl::new(self, Arc::new(conformed_archive_url), path, compression).into()
    }
}
