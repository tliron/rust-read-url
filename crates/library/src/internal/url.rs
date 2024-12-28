use super::{
    super::{context::*, url::*, util::*},
    internal_url::*,
};

use {
    relative_path::*,
    std::{collections::*, path::*},
};

impl URL for InternalUrl {
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
        self.conform_metadata()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(mut url: InternalUrl) -> Result<UrlRef, UrlError> {
            url.conform_metadata()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    fn format(&self) -> Option<String> {
        self.format.clone()
    }

    fn base(&self) -> Option<UrlRef> {
        if self.slashable {
            get_relative_path_parent(&self.path_representation).map(|r| self.new_with(r.into()).into())
        } else {
            self.base_path_representation.as_ref().map(|r| self.new_with(r.clone()).into())
        }
    }

    fn relative(&self, path: &str) -> UrlRef {
        if self.slashable {
            let path = RelativePath::new(&self.path_representation).join(path);
            self.new_with(path.into()).into()
        } else {
            let url_representation = self.path_representation.clone() + path;
            self.new_with(url_representation).into()
        }
    }

    fn key(&self) -> String {
        format!("{}", self)
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use {super::super::errors::*, std::io::Cursor};

        match self.context.internal_url_content(&self.path_representation)? {
            Some(content) => Ok(Box::new(Cursor::new(content))),
            None => Err(UrlError::new_io_not_found(self)),
        }
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenAsyncFuture, crate::UrlError> {
        use {super::super::errors::*, std::io::Cursor};

        async fn open_async(url: InternalUrl) -> Result<AsyncReadRef, UrlError> {
            match url.context.internal_url_content(&url.path_representation)? {
                Some(content) => Ok(Box::pin(Cursor::new(content))),
                None => Err(UrlError::new_io_not_found(url)),
            }
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl InternalUrl {
    fn conform_metadata(&mut self) -> Result<(), crate::UrlError> {
        use super::super::errors::*;

        match self.context.internal_url_metadata(&self.path_representation)? {
            Some((slashable, base_path_representation, format)) => {
                self.slashable = slashable;
                self.base_path_representation = base_path_representation;
                self.format = format;
                Ok(())
            }

            None => Err(UrlError::new_io_not_found(self.to_string())),
        }
    }
}
