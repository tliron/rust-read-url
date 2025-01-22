use super::{
    super::{errors::*, url::*},
    cache::*,
};

use {
    std::{fs::*, io, sync::*},
    tracing::*,
};

impl UrlCache {
    /// Get a cache file from a URL.
    ///
    /// If it already exists returns the path and true. Otherwise copies the URL to a generated path and
    /// returns it and false.
    pub fn file_from(&self, url: &UrlRef, prefix: &str) -> Result<(PathBufRef, bool), UrlError> {
        let key = url.to_string();

        let mut files = self.files.lock()?;
        match files.get(&key) {
            Some(path) => {
                info!("existing file: {}", path.clone().lock()?.display());
                Ok((path.clone(), true))
            }

            None => {
                let path = self.new_path(prefix)?;

                info!("downloading to file (blocking): {}", path.display());
                let mut reader = url.open()?;
                let mut file = File::create_new(path.clone())?;
                io::copy(&mut reader, &mut file)?;

                info!("new file: {}", path.display());
                let path = Arc::new(Mutex::new(path));
                files.insert(key, path.clone());
                Ok((path, false))
            }
        }
    }
}
