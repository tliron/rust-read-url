use super::{
    super::{errors::*, url::*},
    cache::*,
};

use {
    std::sync::*,
    tokio::{fs::*, io},
    tracing::*,
};

impl UrlCache {
    /// Get a cache file from a URL.
    ///
    /// If it already exists returns the path and true. Otherwise copies the URL to a generated path and
    /// returns it and false.
    pub async fn file_from_async(&self, url: &UrlRef, prefix: &str) -> Result<(PathBufRef, bool), UrlError> {
        let key = url.to_string();

        let mut files = self.files.lock()?;
        match files.get(&key) {
            Some(path) => {
                info!("existing file: {}", path.clone().lock()?.to_string_lossy());
                Ok((path.clone(), true))
            }

            None => {
                let path = self.new_path(prefix)?;

                info!("downloading to file (asynchronous): {}", path.to_string_lossy());
                let mut reader = url.open_async()?.await?;
                let mut file = File::create_new(path.clone()).await?;
                io::copy(&mut reader, &mut file).await?;

                info!("new file: {}", path.to_string_lossy());
                let path = Arc::new(Mutex::new(path));
                files.insert(key, path.clone());
                Ok((path, false))
            }
        }
    }
}
