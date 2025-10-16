use super::{super::url::*, cache::*};

use {
    kutil::std::error::*,
    problemo::{common::*, *},
    std::{fs::*, io, sync::*},
    tracing::*,
};

impl UrlCache {
    /// Get a cache file from a URL.
    ///
    /// If it already exists returns the path and true. Otherwise copies the URL to a generated path and
    /// returns it and false.
    pub fn file_from(&self, url: &UrlRef, prefix: &str) -> Result<(PathBufRef, bool), Problem> {
        let key = url.to_string();

        let mut files = self.files.lock().into_thread_problem()?;
        match files.get(&key) {
            Some(path) => {
                info!("existing file: {}", path.clone().lock().into_thread_problem()?.display());
                Ok((path.clone(), true))
            }

            None => {
                let path = self.new_path(prefix)?;

                info!("downloading to file (blocking): {}", path.display());
                let mut reader = io::BufReader::new(url.open()?);
                let mut file =
                    io::BufWriter::new(File::create_new(path.clone()).with_path(path.clone()).via(LowLevelError)?);
                io::copy(&mut reader, &mut file).via(LowLevelError)?;

                info!("new file: {}", path.display());
                let path = Arc::new(Mutex::new(path));
                files.insert(key, path.clone());
                Ok((path, false))
            }
        }
    }
}
