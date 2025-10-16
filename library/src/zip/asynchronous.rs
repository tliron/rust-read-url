use super::{super::errors::*, zip_url::*};

use {
    positioned_io::*,
    problemo::*,
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
    async fn read_zip_move(self) -> Result<AsyncMovableArchiveHandle, Problem>;
}

impl AsyncReadZipMove for Arc<RandomAccessFile> {
    async fn read_zip_move(self) -> Result<AsyncMovableArchiveHandle, Problem> {
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
type DependentArchiveHandle<'file> = ArchiveHandle<'file, RandomAccessFileRef>;

impl AsyncMovableArchiveHandle {
    /// Constructor.
    pub async fn new_for(file: RandomAccessFileRef) -> Result<AsyncMovableArchiveHandle, Problem> {
        AsyncMovableArchiveHandle::try_new(file, async |file| file.read_zip().await.into_url_problem("zip")).await
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
type DependentEntryHandle<'file> = EntryHandle<'file, RandomAccessFileRef>;

impl AsyncMovableArchiveHandle {
    /// A version of [ArchiveHandle::by_name] that returns a [AsyncMovableEntryHandle].
    pub async fn by_name(self, url: &ZipUrl) -> Result<AsyncMovableEntryHandle, Problem> {
        AsyncMovableEntryHandle::try_new(self, async |movable_archive_handle| {
            movable_archive_handle.borrow_dependent().by_name(&url.path).ok_or_else(|| unreachable_url(url, "zip"))
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
type DependentReader<'reader> = Pin<Box<dyn io::AsyncRead + 'reader>>;

impl AsyncMovableEntryHandle {
    /// A version of [EntryHandle::reader] that returns an [AsyncMovableEntryHandleReader].
    pub fn reader(self) -> Result<AsyncMovableEntryHandleReader, Problem> {
        AsyncMovableEntryHandleReader::try_new(self, |movable_entry_handle| {
            Ok(Box::pin(movable_entry_handle.borrow_dependent().reader()))
        })
    }
}

impl io::AsyncRead for AsyncMovableEntryHandleReader {
    fn poll_read(self: Pin<&mut Self>, context: &mut Context, buffer: &mut io::ReadBuf) -> Poll<io::Result<()>> {
        self.get_mut().with_dependent_mut(|_movable_entry_handle, reader| reader.as_mut().poll_read(context, buffer))
    }
}
