use {
    kutil::std::error::*,
    problemo::{common::*, *},
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
        let base_directory = base_directory.unwrap_or_else(|| Self::default_base_directory());

        Self {
            base_directory,
            files: LazyLock::new(|| HashMap::default().into()),
            directories: LazyLock::new(|| HashMap::default().into()),
        }
    }

    /// Default base directory.
    pub fn default_base_directory() -> PathBuf {
        temp_dir().join("read-url")
    }

    /// Resets the cache, deleting all owned files and directories.
    pub fn reset(&self) -> Result<(), Problem> {
        let mut problems = Problems::default();

        let mut files = self.files.lock().into_thread_problem()?;
        for path in files.values() {
            let path = path.lock().into_thread_problem()?;
            info!("deleting file: {}", path.display());
            let path = path.as_path();
            if let Err(error) = remove_file(path) {
                problems.add(error.with_path(path));
            }
        }
        files.clear();

        let mut directories = self.directories.lock().into_thread_problem()?;
        for path in directories.values() {
            let path = path.lock().into_thread_problem()?;
            info!("deleting directory: {}", path.display());
            let path = path.as_path();
            if let Err(error) = remove_dir_all(path) {
                problems.add(error.with_path(path));
            }
        }
        directories.clear();

        if problems.is_empty() { Ok(()) } else { Err(problems.into_problem().via(LowLevelError)) }
    }

    /// Get a cache file.
    ///
    /// If it already exists returns the path and true. Otherwise generates a path and returns it and false.
    pub fn file(&self, key: &str, prefix: &str) -> Result<(PathBufRef, bool), Problem> {
        let key = key.to_string();

        let mut files = self.files.lock().into_thread_problem()?;
        match files.get(&key) {
            Some(path) => {
                info!("existing file: {}", path.clone().lock().into_thread_problem()?.display());
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
    pub fn directory(&self, key: &str, prefix: &str) -> Result<(PathBufRef, bool), Problem> {
        let key = key.to_string();

        let mut directories = self.directories.lock().into_thread_problem()?;
        match directories.get(&key) {
            Some(path) => {
                info!("existing directory: {}", path.clone().lock().into_thread_problem()?.display());
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

    pub(crate) fn new_path(&self, prefix: &str) -> Result<PathBuf, Problem> {
        create_dir_all(&self.base_directory).with_path(&self.base_directory).via(LowLevelError)?;

        // We'll avoid case distinction because Windows doesn't
        let distribution = Uniform::new_inclusive('a', 'z').expect("Uniform::new_inclusive");
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
