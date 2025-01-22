use super::super::errors::*;

use {
    rand::{distr::*, *},
    std::{collections::*, env::*, fs::*, path::*, sync::*},
    tracing::info,
};

const RANDOM_NAME_LENGTH: usize = 32;

/// Common reference type for [PathBuf].
pub type PathBufRef = Arc<Mutex<PathBuf>>;

type PathBufRefMap = LazyLock<Mutex<HashMap<String, PathBufRef>>>;

//
// UrlCache
//

/// Cache for a [UrlContext](super::super::context::UrlContext).
#[derive(Debug)]
pub struct UrlCache {
    /// Base directory.
    pub base_directory: PathBuf,

    /// Files owned by this cache.
    pub files: PathBufRefMap,

    /// Directories owned by this cache.
    pub directories: PathBufRefMap,
}

// type MutexError<'own> = PoisonError<MutexGuard<'own, Vec<String>>>;

impl UrlCache {
    /// Constructor.
    pub fn new(base_directory: Option<PathBuf>) -> Self {
        let base_directory = match base_directory {
            Some(directory) => directory,
            None => Self::default_base_directory(),
        };

        Self {
            base_directory,
            files: LazyLock::new(|| HashMap::new().into()),
            directories: LazyLock::new(|| HashMap::new().into()),
        }
    }

    /// Default base directory.
    pub fn default_base_directory() -> PathBuf {
        temp_dir().join("get-url")
    }

    /// Resets the cache, deleting all owned files and directories.
    pub fn reset(&self) -> Result<(), UrlError> {
        let mut errors = Vec::new();

        let mut files = self.files.lock()?;
        for path in files.values() {
            let path = path.lock()?;
            info!("deleting file: {}", path.display());
            if let Err(error) = remove_file(path.as_path()) {
                errors.push(error);
            }
        }
        files.clear();

        let mut directories = self.directories.lock()?;
        for path in directories.values() {
            let path = path.lock()?;
            info!("deleting directory: {}", path.display());
            if let Err(error) = remove_dir_all(path.as_path()) {
                errors.push(error);
            }
        }
        directories.clear();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(UrlError::IoMany(errors))
        }
    }

    /// Get a cache file.
    ///
    /// If it already exists returns the path and true. Otherwise generates a path and returns it and false.
    pub fn file(&self, key: &str, prefix: &str) -> Result<(PathBufRef, bool), UrlError> {
        let key = key.to_string();

        let mut files = self.files.lock()?;
        match files.get(&key) {
            Some(path) => {
                info!("existing file: {}", path.clone().lock()?.display());
                Ok((path.clone(), true))
            }

            None => {
                let path = self.new_path(prefix)?;
                info!("new file: {}", path.display());
                let path = Arc::new(Mutex::new(path));
                files.insert(key, path.clone());
                Ok((path, false))
            }
        }
    }

    /// Get a cache directory.
    ///
    /// If it already exists returns the path and true. Otherwise creates it ([create_dir_all]) in a
    /// generated path and returns it and false.
    pub fn directory(&self, key: &str, prefix: &str) -> Result<(PathBufRef, bool), UrlError> {
        let key = key.to_string();

        let mut directories = self.directories.lock()?;
        match directories.get(&key) {
            Some(path) => {
                info!("existing driectory: {}", path.clone().lock()?.display());
                Ok((path.clone(), true))
            }

            None => {
                let path = self.new_path(prefix)?;
                info!("new directory: {}", path.display());
                let path = Arc::new(Mutex::new(path));
                directories.insert(key, path.clone());
                Ok((path, false))
            }
        }
    }

    pub(crate) fn new_path(&self, prefix: &str) -> Result<PathBuf, UrlError> {
        create_dir_all(&self.base_directory)?;

        // We'll avoid case distinction because Windows doesn't
        let distribution = Uniform::new_inclusive('a', 'z').unwrap();
        let path: String = rng().sample_iter(distribution).take(RANDOM_NAME_LENGTH).collect();
        let path = prefix.to_string() + &path;
        Ok(self.base_directory.join(path))
    }
}

impl Drop for UrlCache {
    fn drop(&mut self) {
        _ = self.reset();
    }
}
