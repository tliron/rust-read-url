use super::{super::errors::*, zip_url::*};

use {
    positioned_io::*,
    rc_zip_tokio::*,
    self_cell::*,
    std::{future::*, pin::*, sync::*, task::*},
    tokio::io,
};

//
// ReadZipMove
//

/// A version of [ReadZip] that takes ownership of self.
pub trait ReadZipMove {
    /// A version of [ReadZip::read_zip] that takes ownership of self.
    fn read_zip_move(self) -> Result<ArchiveHandleWithFile, UrlError>;
}

impl ReadZipMove for Arc<RandomAccessFile> {
    fn read_zip_move(self) -> Result<ArchiveHandleWithFile, UrlError> {
        todo!();
        //ArchiveHandleWithFile::new_for(self)
    }
}

//
// ArchiveHandleWithFile
//

self_cell!(
    /// An [ArchiveHandle] that owns its [RandomAccessFile].
    pub struct ArchiveHandleWithFile {
        owner: Arc<RandomAccessFile>,

        #[not_covariant]
        dependent: DependentArchiveResult,
    }
);

// self_cell needs a non-nested type name
type DependentArchiveHandleFuture<'own> = Box<dyn Future<Output = DependentArchiveResult<'own>> + 'own>;
type DependentArchiveResult<'own> = Result<DependentArchiveHandle<'own>, rc_zip::error::Error>;
type DependentArchiveHandle<'own> = ArchiveHandle<'own, Arc<RandomAccessFile>>;

impl ArchiveHandleWithFile {
    /// Constructor.
    pub fn new_for(file: Arc<RandomAccessFile>) -> ArchiveHandleWithFile {
        //let archive = file.read_zip().await.unwrap();
        ArchiveHandleWithFile::new(file, |file| {
            Box::pin(async move {
                file.read_zip().await
                //file.read_zip().await;
                // let n: bool;
                // async {
                //     let r = file.read_zip().await.unwrap();
                //     r
                // }
                // let n = async { Box::new(file.read_zip().await) };
                // n
            })
        })
    }
}

//
// EntryHandleWithArchiveHandleWithFile
//

self_cell!(
    /// An [EntryHandle] that owns its [ArchiveHandleWithFile].
    pub struct EntryHandleWithArchiveHandleWithFile {
        owner: ArchiveHandleWithFile,

        #[covariant]
        dependent: DependentEntryHandle,
    }
);

// self_cell needs a non-nested type name
type DependentEntryHandle<'own> = EntryHandle<'own, Arc<RandomAccessFile>>;

impl ArchiveHandleWithFile {
    /// A version of [ArchiveHandle::by_name] that returns an [EntryHandleWithArchiveHandleWithFile].
    pub fn by_name(self, url: &ZipUrl) -> Result<EntryHandleWithArchiveHandleWithFile, UrlError> {
        EntryHandleWithArchiveHandleWithFile::try_new(
            self,
            |archive_handle| -> Result<DependentEntryHandle, UrlError> {
                // async {
                //     archive_handle.with_dependent(|file, archive_handle| match archive_handle.by_name(&url.path) {
                //         Some(entry) => Ok(entry),
                //         None => Err(UrlError::new_io_not_found(url)),
                //     })
                // };
                todo!();
            },
        )
    }
}

//
// EntryHandleWithArchiveHandleWithFileReader
//

self_cell!(
    /// An [io::Read] that owns its [EntryHandleWithArchiveHandleWithFile].
    pub struct EntryHandleWithArchiveHandleWithFileReader {
        owner: EntryHandleWithArchiveHandleWithFile,

        #[covariant]
        dependent: DependentReader,
    }
);

// self_cell needs a non-nested type name
type DependentReader<'own> = Box<dyn io::AsyncRead + Unpin + 'own>;

impl EntryHandleWithArchiveHandleWithFile {
    /// A version of [EntryHandle::reader] that returns an [EntryHandleWithArchiveHandleWithFileReader].
    pub fn reader(self) -> Result<EntryHandleWithArchiveHandleWithFileReader, UrlError> {
        EntryHandleWithArchiveHandleWithFileReader::try_new(self, |entry_handle| -> Result<DependentReader, UrlError> {
            Ok(Box::new(entry_handle.borrow_dependent().reader()))
        })
    }
}

impl io::AsyncRead for EntryHandleWithArchiveHandleWithFileReader {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut io::ReadBuf<'_>) -> Poll<io::Result<()>> {
        let d = self.borrow_dependent();
        //self.with_dependent_mut(|_entry_handle, reader| reader.poll_read(cx, buf))
        todo!();
    }
}
