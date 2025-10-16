use super::{
    super::{context::*, url::*, util::*},
    http_url::*,
};

use relative_path::*;

impl URL for HttpUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn cloned(&self) -> UrlRef {
        self.clone().into()
    }

    fn query(&self) -> Option<UrlQuery> {
        url_query(&self.url)
    }

    fn fragment(&self) -> Option<String> {
        url_fragment(&self.url)
    }

    fn format(&self) -> Option<String> {
        // TODO: from MIME type?
        get_format_from_path(self.url.path())
    }

    fn base(&self) -> Option<UrlRef> {
        get_relative_path_parent(self.url.path()).map(|path| self.new_with(path.as_str()).into())
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(RelativePath::new(self.url.path()).join(path).as_str()).into()
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), problemo::Problem> {
        use super::super::errors::*;

        let tokio = runtime()?;
        // TODO: can we get the MIME type here for format?
        let response =
            tokio.block_on(self.context.http_client.head(self.url.clone()).send()).into_url_problem("http")?;
        if response.status().is_success() { Ok(()) } else { Err(unreachable_url(self, "http")) }
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, problemo::Problem> {
        use {super::super::errors::*, problemo::*};

        async fn conform_async(url: HttpUrl) -> Result<UrlRef, Problem> {
            let response = url.context.http_client.head(url.url.clone()).send().await.into_url_problem("http")?;
            if response.status().is_success() {
                Ok(url.into())
            } else {
                Err(unreachable_url(url.url.as_str(), "internal"))
            }
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, problemo::Problem> {
        use {
            super::super::errors::*,
            kutil::io::stream::{bytes::*, *},
        };

        let runtime = runtime()?;
        let response =
            runtime.block_on(self.context.http_client.get(self.url.clone()).send()).into_url_problem("http")?;
        let stream = response.bytes_stream();
        let reader = BlockingBytesStreamReader::new(BlockingStream::new(stream, runtime));
        Ok(Box::new(reader))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, problemo::Problem> {
        use {super::super::errors::*, kutil::io::stream::bytes::*, problemo::*};

        async fn open_async(url: HttpUrl) -> Result<AsyncReadRef, Problem> {
            let response = url.context.http_client.get(url.url.clone()).send().await.into_url_problem("http")?;
            let stream = response.bytes_stream();
            let reader = AsyncBytesStreamReader::new(stream);
            Ok(Box::pin(reader))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(feature = "blocking")]
fn runtime() -> Result<tokio::runtime::Runtime, problemo::Problem> {
    use super::super::errors::*;
    Ok(tokio::runtime::Builder::new_current_thread().enable_all().build().into_url_problem("http")?)
}
