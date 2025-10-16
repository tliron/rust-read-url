use super::super::error::*;

use {
    ouroboros::*,
    positioned_io::*,
    rc_zip_tokio::*,
    std::{pin::*, sync::*, task::*},
    tokio::io,
};

//
// AsyncZipReader
//

/// Async zip reader.
pub struct AsyncZipReader {
    internal: AsyncZipReaderInternal,
}

impl AsyncZipReader {
    /// Constructor.
    pub async fn new(file: Arc<RandomAccessFile>, path: &str, url_representation: &str) -> Result<Self, Problem> {
        let path = path.to_string();
        let url_representation = url_representation.to_string();

        Ok(Self {
            internal: AsyncZipReaderInternalAsyncTryBuilder::<_, _, _, Problem> {
                file,

                archive_builder: |file: &Arc<RandomAccessFile>| Box::pin(async move { Ok(file.read_zip().await?) }),

                entry_builder: |archive: &ArchiveHandle<'_, Arc<RandomAccessFile>>| {
                    Box::pin(async move {
                        match archive.by_name(path) {
                            Some(entry) => Ok(entry),
                            None => Err(ProblemContext::new_io_not_found(url_representation)),
                        }
                    })
                },

                reader_builder: |entry: &EntryHandle<'_, Arc<RandomAccessFile>>| {
                    Box::pin(async move {
                        let reader: Box<dyn io::AsyncRead + Unpin + '_> = Box::new(entry.reader());
                        Ok(Pin::new(reader))
                    })
                },
            }
            .try_build()
            .await?,
        })
    }
}

impl io::AsyncRead for AsyncZipReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.internal.with_reader_mut(|reader| {
            let reader = reader.as_mut();
            reader.poll_read(cx, buf)
        })
    }
}

#[self_referencing]
struct AsyncZipReaderInternal {
    file: Arc<RandomAccessFile>,

    #[borrows(file)]
    #[covariant]
    archive: ArchiveHandle<'this, Arc<RandomAccessFile>>,

    #[borrows(archive)]
    #[covariant]
    entry: EntryHandle<'this, Arc<RandomAccessFile>>,

    #[borrows(entry)]
    #[covariant]
    reader: Pin<Box<dyn io::AsyncRead + Unpin + 'this>>,
}
