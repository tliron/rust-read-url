use super::{super::errors::*, zip_url::*};

use {
    problemo::*,
    rc_zip_sync::*,
    self_cell::*,
    std::{fs::*, io},
};

//
// ReadZipMove
//

/// A version of [ReadZip] that takes ownership of self.
pub trait ReadZipMove {
    /// A version of [ReadZip::read_zip] that takes ownership of self.
    fn read_zip_move(self) -> Result<MovableArchiveHandle, Problem>;
}

impl ReadZipMove for File {
    fn read_zip_move(self) -> Result<MovableArchiveHandle, Problem> {
        MovableArchiveHandle::new_for(self)
    }
}

//
// MovableArchiveHandle
//

self_cell!(
    /// An [ArchiveHandle] that owns its [File].
    pub struct MovableArchiveHandle {
        owner: File,

        #[covariant]
        dependent: DependentArchiveHandle,
    }
);

// self_cell needs a non-nested type name
type DependentArchiveHandle<'file> = ArchiveHandle<'file, File>;

impl MovableArchiveHandle {
    /// Constructor.
    pub fn new_for(file: File) -> Result<MovableArchiveHandle, Problem> {
        MovableArchiveHandle::try_new(file, |file| file.read_zip().into_url_problem("zip"))
    }
}

//
// MovableEntryHandle
//

self_cell!(
    /// An [EntryHandle] that owns its [MovableArchiveHandle].
    pub struct MovableEntryHandle {
        owner: MovableArchiveHandle,

        #[covariant]
        dependent: DependentEntryHandle,
    }
);

// self_cell needs a non-nested type name
type DependentEntryHandle<'file> = EntryHandle<'file, File>;

impl MovableArchiveHandle {
    /// A version of [ArchiveHandle::by_name] that returns a [MovableEntryHandle].
    pub fn by_name(self, url: &ZipUrl) -> Result<MovableEntryHandle, Problem> {
        MovableEntryHandle::try_new(self, |movable_archive_handle| {
            movable_archive_handle.borrow_dependent().by_name(&url.path).ok_or_else(|| unreachable_url(url, "zip"))
        })
    }
}

//
// MovableEntryHandleReader
//

self_cell!(
    /// An [io::Read] that owns its [MovableEntryHandle].
    pub struct MovableEntryHandleReader {
        owner: MovableEntryHandle,

        #[covariant]
        dependent: DependentReader,
    }
);

// self_cell needs a non-nested type name
type DependentReader<'reader> = Box<dyn io::Read + Send + Sync + 'reader>;

impl MovableEntryHandle {
    /// A version of [EntryHandle::reader] that returns a [MovableEntryHandleReader].
    pub fn reader(self) -> Result<MovableEntryHandleReader, Problem> {
        MovableEntryHandleReader::try_new(self, |movable_entry_handle| {
            Ok(Box::new(movable_entry_handle.borrow_dependent().reader()))
        })
    }
}

impl io::Read for MovableEntryHandleReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.with_dependent_mut(|_entry_handle, reader| reader.read(buffer))
    }
}
