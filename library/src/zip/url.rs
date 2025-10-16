use super::{
    super::{context::*, url::*, util::*},
    zip_url::*,
};

impl URL for ZipUrl {
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
        get_relative_path_parent(&self.path).map(|path| self.new_with(path).into())
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(self.path.join(path)).into()
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), problemo::Problem> {
        // (We assume the archive URL has already been conformed)

        // Note that ZIP entries could have relative or absolute paths
        // (though absolute paths are rare), so we cannot conform to absolute
        self.path = self.path.normalize();

        self.open()?;

        Ok(())
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, problemo::Problem> {
        use problemo::*;

        async fn conform_async(mut url: ZipUrl) -> Result<UrlRef, Problem> {
            // (We assume the archive URL has already been conformed)

            // Note that ZIP entries could have relative or absolute paths
            // (though absolute paths are rare), so we cannot conform to absolute
            url.path = url.path.normalize();

            let _ = url.open_async()?;

            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, problemo::Problem> {
        use {
            super::blocking::*,
            kutil::std::error::*,
            problemo::{common::*, *},
            std::{fs::*, sync::*},
        };

        let archive_path = match self.archive_url.local() {
            Some(path) => Mutex::new(path).into(),

            None => {
                let (path, _) = self.context.cache.file_from(&self.archive_url, "zip-")?;
                path
            }
        };

        let archive_path = archive_path.lock().into_thread_problem()?;

        let file = File::open(archive_path.clone()).with_path(archive_path.clone()).via(LowLevelError)?;
        let archive = file.read_zip_move()?;
        let entry = archive.by_name(self)?;
        Ok(Box::new(entry.reader()?))

        // Read all:
        //
        // let archive = file.read_zip()?;
        // if let Some(entry) = archive.by_name(&self.path) {
        //     // We can't detach the reader, so must read all the bytes here :(
        //     // let bytes = entry.bytes()?;
        //     // return Ok(Box::new(Cursor::new(bytes)));
        // }
        // Err(ProblemContext::new_io_not_found(self))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, problemo::Problem> {
        use {
            super::asynchronous::*,
            kutil::std::error::*,
            positioned_io::*,
            problemo::{common::*, *},
            std::sync::*,
        };

        async fn open_async(url: ZipUrl) -> Result<AsyncReadRef, Problem> {
            let archive_path = match url.archive_url.local() {
                Some(path) => Mutex::new(path).into(),

                None => {
                    let (path, _) = url.context.cache.file_from_async(&url.archive_url, "zip-").await?;
                    path
                }
            };

            let archive_path = archive_path.lock().into_thread_problem()?;

            let file = Arc::new(
                RandomAccessFile::open(archive_path.clone()).with_path(archive_path.clone()).via(LowLevelError)?,
            );
            let archive = file.read_zip_move().await?;
            let entry = archive.by_name(&url).await?;
            Ok(Box::pin(entry.reader()?))

            // Read all:
            //
            // let archive = file.read_zip().await?;
            // if let Some(entry) = archive.by_name(&url.path) {
            //     // We can't detach the reader, so must read all the bytes here :(
            //     let bytes = entry.bytes().await?;
            //     return Ok(Box::pin(Cursor::new(bytes)));
            // }
            // Err(ProblemContext::new_io_not_found(url))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}
