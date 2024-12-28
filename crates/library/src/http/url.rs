use super::{
    super::{context::*, url::*, util::*},
    http_url::*,
};

use {
    relative_path::*,
    std::{collections::*, path::*},
};

impl URL for HttpUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        url_query(&self.url)
    }

    fn fragment(&self) -> Option<String> {
        url_fragment(&self.url)
    }

    fn local(&self) -> Option<PathBuf> {
        None
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), crate::UrlError> {
        use super::super::errors::*;

        let tokio = tokio()?;
        // TODO: can we get the MIME type here for format?
        let response = tokio.block_on(self.context.http_client.head(self.url.clone()).send())?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(UrlError::new_io_not_found(self))
        }
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(url: HttpUrl) -> Result<UrlRef, UrlError> {
            let response = url.context.http_client.head(url.url.clone()).send().await?;
            if response.status().is_success() {
                Ok(url.into())
            } else {
                Err(UrlError::new_io_not_found(url.url.as_str()))
            }
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    fn format(&self) -> Option<String> {
        // TODO: from MIME type?
        get_format_from_path(self.url.path())
    }

    fn base(&self) -> Option<UrlRef> {
        get_relative_path_parent(self.url.path()).map(|p| self.new_with(p.as_str()).into())
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(RelativePath::new(self.url.path()).join(path).as_str()).into()
    }

    fn key(&self) -> String {
        format!("{}", self)
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use kutil_io::{bytes_stream::*, stream::*};

        let tokio = tokio()?;
        let response = tokio.block_on(self.context.http_client.get(self.url.clone()).send())?;
        let stream = BlockingStream::new(response.bytes_stream(), tokio);
        let reader = BlockingBytesStreamReader::new(stream);
        Ok(Box::new(reader))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenAsyncFuture, crate::UrlError> {
        use {super::super::errors::*, kutil_io::bytes_stream::*};

        async fn open_async(url: HttpUrl) -> Result<AsyncReadRef, UrlError> {
            let response = url.context.http_client.get(url.url.clone()).send().await?;
            let stream = response.bytes_stream();
            let reader = AsyncBytesStreamReader::new(stream);
            Ok(Box::pin(reader))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(feature = "blocking")]
fn tokio() -> Result<tokio::runtime::Runtime, crate::UrlError> {
    Ok(tokio::runtime::Builder::new_current_thread().enable_all().build()?)
}
