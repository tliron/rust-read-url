use super::{
    super::{context::*, url::*, util::*},
    file_url::*,
};

use std::path::*;

impl URL for FileUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<UrlQuery> {
        self.query.clone()
    }

    fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    fn format(&self) -> Option<String> {
        get_format_from_path(&self.path.to_string_lossy())
    }

    fn local(&self) -> Option<PathBuf> {
        Some(self.path.clone())
    }

    fn base(&self) -> Option<UrlRef> {
        self.path.parent().map(|path| {
            let mut path = path.to_string_lossy();
            path += MAIN_SEPARATOR_STR;
            self.new_with(path.to_string().into()).into()
        })
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(self.path.join(path)).into()
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), crate::UrlError> {
        self.conform_path()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(mut url: FileUrl) -> Result<UrlRef, UrlError> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use std::fs::*;

        Ok(Box::new(File::open(self.path.clone())?))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, crate::UrlError> {
        use {super::super::errors::*, tokio::fs::*};

        async fn open_async(url: FileUrl) -> Result<AsyncReadRef, UrlError> {
            let file = File::open(url.path).await?;
            Ok(Box::pin(file))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl FileUrl {
    fn conform_path(&mut self) -> Result<(), crate::UrlError> {
        use {super::super::errors::*, std::io};

        self.path = match conform_file_path(&self.path) {
            Ok(path) => path,
            Err(error) => {
                if error.kind() == io::ErrorKind::NotFound {
                    return Err(UrlError::new_io_not_found(self));
                }
                return Err(error.into());
            }
        };
        Ok(())
    }
}
