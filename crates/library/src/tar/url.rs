use super::{
    super::{context::*, url::*, util::*},
    tar_url::*,
};

impl URL for TarUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<UrlQuery> {
        self.archive_url.query()
    }

    fn fragment(&self) -> Option<String> {
        self.archive_url.fragment()
    }

    fn format(&self) -> Option<String> {
        get_format_from_path(&self.path)
    }

    fn base(&self) -> Option<UrlRef> {
        get_relative_path_parent(&self.path).map(|p| self.new_with(p).into())
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

        async fn conform_async(mut url: TarUrl) -> Result<UrlRef, UrlError> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use {
            super::{super::errors::*, compression::*},
            gix::bstr::*,
            kutil_io::reader::*,
            tar::*,
        };

        let mut reader = self.archive_url.open()?;

        // Decompression
        match self.get_compression() {
            TarCompression::None => {}
            #[cfg(feature = "gzip")]
            TarCompression::GZip => {
                use {flate2::read::*, tracing::info};
                info!("gzip decompression (blocking)");
                reader = Box::new(GzDecoder::new(reader));
            }
            #[cfg(feature = "zstd")]
            TarCompression::Zstd => {
                use {tracing::info, zstd::stream::*};
                info!("zstd decompression (blocking)");
                reader = Box::new(Decoder::new(reader)?);
            }
            #[cfg(not(all(feature = "gzip", feature = "zstd")))]
            compression => return Err(UrlError::UnsupportedFormat(compression.to_string())),
        }

        let mut archive = Archive::new(reader);

        // Advance the reader to the beginning of the tar entry
        let mut size = None;
        for entry in archive.entries()? {
            let entry = entry?;
            match entry.path_bytes().to_str() {
                Ok(path) => {
                    if path == self.path {
                        size = Some(entry.size() as usize);
                        break;
                    }
                }

                Err(_) => {}
            }
        }

        // It might seem like an unreliable trick to assume that we are at the right
        // place with the reader; after all this is undocumented, internal behavior of
        // the "tar" crate.

        // However, it is 1) correct!, 2) a reasonable expectation considering the
        // crate's *external* design surface, and 3) our only real choice (aside from
        // going with "unsafe") because we cannot otherwise disentangle the references
        // of Entry to Entries to Archive and return a movable io::Read to an Entry.

        match size {
            Some(size) => {
                // Get our reader back, now at the right place
                let reader = archive.into_inner();

                // BoundedReader will make sure we don't read beyond our entry
                Ok(Box::new(BoundedReader::new(reader, size)))
            }

            None => Err(UrlError::new_io_not_found(self)),
        }
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, crate::UrlError> {
        use {
            super::{super::errors::*, compression::*},
            futures::*,
            gix::bstr::*,
            tokio_tar::*,
        };

        // Note that we are using a fork of async-tar that uses Tokio instead of Futures
        // Let's hope it stays maintained! Otherwise, we could also use tokio-util Compat
        // with async-tar.

        async fn open_async(url: TarUrl) -> Result<AsyncReadRef, UrlError> {
            let mut reader = url.archive_url.open_async()?.await?;

            // Decompression
            match url.get_compression() {
                TarCompression::None => {}
                #[cfg(feature = "gzip")]
                TarCompression::GZip => {
                    use {async_compression::tokio::bufread::*, tokio::io::*, tracing::info};
                    info!("gzip decompression (asynchronous)");
                    reader = Box::pin(GzipDecoder::new(BufReader::new(reader)));
                }
                #[cfg(feature = "zstd")]
                TarCompression::Zstd => {
                    use {async_compression::tokio::bufread::*, tokio::io::*, tracing::info};
                    info!("zstd decompression (asynchronous)");
                    reader = Box::pin(ZstdDecoder::new(BufReader::new(reader)));
                }
                #[cfg(not(all(feature = "gzip", feature = "zstd")))]
                compression => return Err(UrlError::UnsupportedFormat(compression.to_string())),
            }

            let mut archive = Archive::new(reader);

            let mut entries = archive.entries()?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                match entry.path_bytes().to_str() {
                    Ok(path) => {
                        if path == url.path {
                            return Ok(Box::pin(entry));
                        }
                    }

                    Err(_) => {}
                }
            }

            return Err(UrlError::new_io_not_found(url));
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl TarUrl {
    fn conform_path(&mut self) -> Result<(), crate::UrlError> {
        // (We assume the archive URL has already been conformed)

        // Note that tar entries could have relative or absolute paths
        // (though absolute paths are rare), so we cannot conform to absolute
        self.path = self.path.normalize();

        Ok(())
    }
}
