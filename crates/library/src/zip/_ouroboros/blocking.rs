use super::{super::errors::*, zip_url::*};

use {
    ouroboros::*,
    rc_zip_sync::*,
    std::{fs::*, io},
};

//
// ZipReader
//

/// Zip reader.
pub struct ZipReader {
    internal: ZipReaderInternal,
}

impl ZipReader {
    /// Constructor.
    pub fn new(url: &ZipUrl) -> Result<Self, UrlError> {
        let archive_path = match url.archive_url.local() {
            Some(path) => path,

            None => {
                let (path, _) = url.context.cache.file_from(&url.archive_url, "zip-")?;
                path
            }
        };

        let file = File::open(archive_path)?;

        let entry_path = url.path.as_str();

        Ok(Self {
            internal: ZipReaderInternalTryBuilder {
                file,

                archive_builder: |file: &File| -> Result<ArchiveHandle<'_, File>, UrlError> { Ok(file.read_zip()?) },

                entry_builder: |archive: &ArchiveHandle<'_, File>| -> Result<EntryHandle<'_, File>, UrlError> {
                    archive.by_name(entry_path).ok_or_else(|| UrlError::new_io_not_found(url))
                },

                reader_builder: |entry: &EntryHandle<'_, File>| -> Result<Box<dyn io::Read + '_>, UrlError> {
                    Ok(Box::new(entry.reader()))
                },
            }
            .try_build()?,
        })
    }
}

impl io::Read for ZipReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.internal.with_reader_mut(|reader| reader.read(buf))
    }
}

#[self_referencing]
struct ZipReaderInternal {
    file: File,

    #[borrows(file)]
    #[covariant]
    archive: ArchiveHandle<'this, File>,

    #[borrows(archive)]
    #[covariant]
    entry: EntryHandle<'this, File>,

    #[borrows(entry)]
    #[covariant]
    reader: Box<dyn io::Read + 'this>,
}
