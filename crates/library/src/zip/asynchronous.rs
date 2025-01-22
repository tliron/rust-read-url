use super::{super::errors::*, zip_url::*};

use {
    positioned_io::*,
    rc_zip_tokio::*,
    self_cell::*,
    std::{pin::*, sync::*, task::*},
    tokio::io,
};

type RandomAccessFileRef = Arc<RandomAccessFile>;

//
// AsyncReadZipMove
//

/// A version of [ReadZip] that takes ownership of self.
pub trait AsyncReadZipMove {
    /// A version of [ReadZip::read_zip] that takes ownership of self.
    #[allow(async_fn_in_trait)]
    async fn read_zip_move(self) -> Result<AsyncMovableArchiveHandle, UrlError>;
}

impl AsyncReadZipMove for Arc<RandomAccessFile> {
    async fn read_zip_move(self) -> Result<AsyncMovableArchiveHandle, UrlError> {
        AsyncMovableArchiveHandle::new_for(self).await
    }
}

//
// AsyncMovableArchiveHandle
//

self_cell!(
    /// An [ArchiveHandle] that owns its [RandomAccessFile].
    pub struct AsyncMovableArchiveHandle {
        owner: RandomAccessFileRef,

        #[covariant, async_builder]
        dependent: DependentArchiveHandle,
    }
);

// self_cell needs a non-nested type name
type DependentArchiveHandle<'own> = ArchiveHandle<'own, RandomAccessFileRef>;

impl AsyncMovableArchiveHandle {
    /// Constructor.
    pub async fn new_for(file: RandomAccessFileRef) -> Result<AsyncMovableArchiveHandle, UrlError> {
        AsyncMovableArchiveHandle::try_new(file, async |file| file.read_zip().await.map_err(|e| e.into())).await
    }
}

//
// AsyncMovableEntryHandle
//

self_cell!(
    /// An [EntryHandle] that owns its [AsyncMovableArchiveHandle].
    pub struct AsyncMovableEntryHandle {
        owner: AsyncMovableArchiveHandle,

        #[covariant, async_builder]
        dependent: DependentEntryHandle,
    }
);

// self_cell needs a non-nested type name
type DependentEntryHandle<'own> = EntryHandle<'own, RandomAccessFileRef>;

impl AsyncMovableArchiveHandle {
    /// A version of [ArchiveHandle::by_name] that returns a [MovableEntryHandle].
    pub async fn by_name(self, url: &ZipUrl) -> Result<AsyncMovableEntryHandle, UrlError> {
        AsyncMovableEntryHandle::try_new(self, async |movable_archive_handle| {
            match movable_archive_handle.borrow_dependent().by_name(&url.path) {
                Some(entry) => Ok(entry),
                None => Err(UrlError::new_io_not_found(url)),
            }
        })
        .await
    }
}

//
// AsyncMovableEntryHandleReader
//

self_cell!(
    /// An [io::AsyncRead] that owns its [AsyncMovableEntryHandle].
    pub struct AsyncMovableEntryHandleReader {
        owner: AsyncMovableEntryHandle,

        #[covariant]
        dependent: DependentReader,
    }
);

// self_cell needs a non-nested type name
type DependentReader<'own> = Pin<Box<dyn io::AsyncRead + 'own>>;

impl AsyncMovableEntryHandle {
    /// A version of [EntryHandle::reader] that returns an [AsyncMovableEntryHandleReader].
    pub fn reader(self) -> Result<AsyncMovableEntryHandleReader, UrlError> {
        AsyncMovableEntryHandleReader::try_new(self, |movable_entry_handle| {
            Ok(Box::pin(movable_entry_handle.borrow_dependent().reader()))
        })
    }
}

impl io::AsyncRead for AsyncMovableEntryHandleReader {
    fn poll_read(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.get_mut().with_dependent_mut(|_movable_entry_handle, reader| reader.as_mut().poll_read(context, buffer))
    }
}
