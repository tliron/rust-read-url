use super::{
    super::{context::*, url::*, util::*},
    internal_url::*,
};

use relative_path::*;

impl URL for InternalUrl {
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
        self.metadata.format.clone()
    }

    fn base(&self) -> Option<UrlRef> {
        if self.metadata.slashable {
            get_relative_path_parent(&self.path).map(|path| self.new_with(path.into()).into())
        } else {
            self.metadata.base_path.as_ref().map(|path| self.new_with(path.clone()).into())
        }
    }

    fn relative(&self, path: &str) -> UrlRef {
        if self.metadata.slashable {
            let path = RelativePath::new(&self.path).join(path);
            self.new_with(path.into()).into()
        } else {
            let path = self.path.clone() + path;
            self.new_with(path).into()
        }
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), super::super::UrlError> {
        self.conform_metadata()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, super::super::UrlError> {
        use super::super::errors::*;

        async fn conform_async(mut url: InternalUrl) -> Result<UrlRef, UrlError> {
            url.conform_metadata()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, super::super::UrlError> {
        use super::super::errors::*;

        let reader = self.context.read_internal_url(&self.path)?.ok_or_else(|| UrlError::new_io_not_found(self))?;
        Ok(Box::new(reader))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, super::super::UrlError> {
        use super::super::errors::*;

        async fn open_async(url: InternalUrl) -> Result<AsyncReadRef, UrlError> {
            let reader = url.context.read_internal_url(&url.path)?.ok_or_else(|| UrlError::new_io_not_found(url))?;
            Ok(Box::pin(reader))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl InternalUrl {
    fn conform_metadata(&mut self) -> Result<(), super::super::UrlError> {
        use super::super::errors::*;

        let metadata = self
            .context
            .internal_url_metadata(&self.path)?
            .ok_or_else(|| UrlError::new_io_not_found(self.to_string()))?;

        self.metadata = metadata;
        Ok(())
    }
}
