use super::{
    super::{context::*, url::*, util::*},
    mock_url::*,
};

use relative_path::*;

impl URL for MockUrl {
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
        self.format.clone()
    }

    fn base(&self) -> Option<UrlRef> {
        if self.slashable {
            get_relative_path_parent(&self.url_representation).map(|path| self.new_with(path.into()).into())
        } else {
            self.base_url_representation.as_ref().map(|path| self.new_with(path.clone()).into())
        }
    }

    fn relative(&self, path: &str) -> UrlRef {
        if self.slashable {
            let path = RelativePath::new(&self.url_representation).join(path);
            self.new_with(path.into()).into()
        } else {
            let url_representation = self.url_representation.clone() + path;
            self.new_with(url_representation).into()
        }
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), super::super::UrlError> {
        Ok(())
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, super::super::UrlError> {
        use super::super::errors::*;

        async fn conform_async(url: MockUrl) -> Result<UrlRef, UrlError> {
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, super::super::UrlError> {
        use super::super::errors::*;

        let content = self.content.as_ref().ok_or_else(|| UrlError::new_io_not_found(self))?;
        Ok(Box::new(content.reader()))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, super::super::UrlError> {
        use super::super::errors::*;

        async fn open_async(url: MockUrl) -> Result<AsyncReadRef, UrlError> {
            match url.content {
                Some(content) => Ok(Box::pin(content.reader())),
                None => Err(UrlError::new_io_not_found(url)),
            }
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}
