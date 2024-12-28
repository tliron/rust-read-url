use super::{
    super::{context::*, url::*, util::*},
    zip_url::*,
};

use std::{collections::*, path::*};

impl URL for ZipUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        self.archive_url.query()
    }

    fn fragment(&self) -> Option<String> {
        self.archive_url.fragment()
    }

    fn local(&self) -> Option<PathBuf> {
        None
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), crate::UrlError> {
        self.conform_path()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(mut url: ZipUrl) -> Result<UrlRef, UrlError> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
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

    fn key(&self) -> String {
        format!("{}", self)
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        use {
            super::{super::errors::*, blocking::*},
            std::{fs::*, sync::*},
        };

        let archive_path = match self.archive_url.local() {
            Some(path) => Arc::new(Mutex::new(path)),

            None => {
                let (path, _) = self.context.cache.file_from(&self.archive_url, "zip-")?;
                path
            }
        };

        let archive_path = archive_path.lock().map_err(|e| UrlError::Mutex(e.to_string()))?;

        let file = File::open(archive_path.clone())?;
        let archive = file.read_zip_move()?;
        let entry = archive.by_name(self)?;
        Ok(Box::new(entry.reader()?))

        // let archive = file.read_zip()?;
        // if let Some(entry) = archive.by_name(&self.path) {
        //     // We can't detatch the reader, so must read all the bytes here :(
        //     // let bytes = entry.bytes()?;
        //     // return Ok(Box::new(Cursor::new(bytes)));
        // }
        // Err(UrlError::new_io_not_found(self))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenAsyncFuture, crate::UrlError> {
        use {
            super::{super::errors::*, asynchronous::*},
            positioned_io::*,
            std::sync::*,
        };

        async fn open_async(url: ZipUrl) -> Result<AsyncReadRef, UrlError> {
            let archive_path = match url.archive_url.local() {
                Some(path) => Arc::new(Mutex::new(path)),

                None => {
                    let (path, _) = url.context.cache.file_from_async(&url.archive_url, "zip-").await?;
                    path
                }
            };

            let archive_path = archive_path.lock().map_err(|e| UrlError::Mutex(e.to_string()))?;

            let file = Arc::new(RandomAccessFile::open(archive_path.clone())?);
            let r = AsyncZipReader::new(file, url.path.as_str(), &url.to_string()).await?;
            return Ok(Box::pin(r));

            // let archive = file.read_zip().await?;
            // if let Some(entry) = archive.by_name(&url.path) {
            //     // We can't detatch the reader, so must read all the bytes here :(
            //     let bytes = entry.bytes().await?;
            //     return Ok(Box::pin(Cursor::new(bytes)));
            // }
            // Err(UrlError::new_io_not_found(url))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl ZipUrl {
    fn conform_path(&mut self) -> Result<(), crate::UrlError> {
        // (We assume the archive URL has already been conformed)

        // Note that zip entries could have relative or absolute paths
        // (though absolute paths are rare), so we cannot conform to absolute
        self.path = self.path.normalize();

        Ok(())
    }
}

// #[cfg(all(feature = "blocking", not(feature = "zip-rc")))]
// fn open(&self) -> Result<ReadRef, crate::UrlError> {
//     use {
//         memmap2::*,
//         piz::read::*,
//         std::fs::*,
//     };

//     let path = match self.archive_url.local() {
//         Some(path) => path,

//         None => {
//             let (path, _) = self.context.cache.file_from(&self.archive_url, "zip-")?;
//             path
//         }
//     };

//     let file = File::open(path)?;
//     let mapping = unsafe { Mmap::map(&file)? };
//     let archive = ZipArchive::new(&mapping)?;
//     let tree = as_tree(archive.entries())?;
//     let metadata = tree.lookup(self.path.to_string_lossy().into_owned())?;
//     let reader = archive.read(metadata)?;
//     Ok(Box::new(reader))
// }
