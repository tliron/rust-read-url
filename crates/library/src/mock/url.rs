use super::{
    super::{context::*, url::*, util::*},
    mock_url::*,
};

use {
    relative_path::*,
    std::{collections::*, path::*},
};

impl URL for MockUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        self.query.clone()
    }

    fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    fn local(&self) -> Option<PathBuf> {
        None
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), crate::UrlError> {
        Ok(())
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(url: MockUrl) -> Result<UrlRef, UrlError> {
            Ok(url.into())
        }

        // Cloning the content is not great, but here we are optimizing for straightforwardness
        Ok(Box::pin(conform_async(self.clone())))
    }

    fn format(&self) -> Option<String> {
        self.format.clone()
    }

    fn base(&self) -> Option<UrlRef> {
        if self.slashable {
            get_relative_path_parent(&self.url_representation).map(|r| self.new_with(r.into()).into())
        } else {
            self.base_url_representation.as_ref().map(|r| self.new_with(r.clone()).into())
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

    fn key(&self) -> String {
        format!("{}", self)
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use {super::super::errors::*, std::io::Cursor};

        match &self.content {
            // Cloning the content is not great, but here we are optimizing for straightforwardness
            Some(content) => Ok(Box::new(Cursor::new(content.clone()))),
            None => Err(UrlError::new_io_not_found(self)),
        }
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenAsyncFuture, crate::UrlError> {
        use {super::super::errors::*, std::io::Cursor};

        async fn open_async(url: MockUrl) -> Result<AsyncReadRef, UrlError> {
            match url.content {
                Some(content) => Ok(Box::pin(Cursor::new(content))),
                None => Err(UrlError::new_io_not_found(url)),
            }
        }

        // Cloning the content is not great, but here we are optimizing for straightforwardness
        Ok(Box::pin(open_async(self.clone())))
    }
}
