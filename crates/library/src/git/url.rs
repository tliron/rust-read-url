use super::{
    super::{context::*, url::*, util::*},
    git_url::*,
};

use std::{collections::*, path::*};

impl URL for GitUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        self.repository_url.query()
    }

    fn fragment(&self) -> Option<String> {
        self.repository_url.fragment()
    }

    fn local(&self) -> Option<PathBuf> {
        None
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), crate::UrlError> {
        self.conform_path()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn conform_async(mut url: GitUrl) -> Result<UrlRef, UrlError> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    fn format(&self) -> Option<String> {
        get_format_from_path(&self.path)
    }

    fn base(&self) -> Option<UrlRef> {
        get_relative_path_parent(&self.path).map(|p| self.new_with(p).into())
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(self.path.join(path)).into()
    }

    fn key(&self) -> String {
        format!("{}", self)
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, crate::UrlError> {
        Ok(Box::new(self.open_cursor()?))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenAsyncFuture, crate::UrlError> {
        use super::super::errors::*;

        async fn open_async(url: GitUrl) -> Result<AsyncReadRef, UrlError> {
            Ok(Box::pin(url.open_cursor()?))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl GitUrl {
    fn open_cursor(&self) -> Result<std::io::Cursor<Vec<u8>>, crate::UrlError> {
        use {
            super::{super::errors::*, errors::*},
            gix::*,
            std::{io::Cursor, num::*},
            tracing::info,
        };

        // Interpret fragment
        let (commit, ref_name) = match self.fragment() {
            Some(fragment) => {
                // Is it a commit hash?
                match ObjectId::from_hex(fragment.as_bytes()) {
                    Ok(object) => {
                        info!("using commit: {}", object);
                        (Some(object), None)
                    }

                    // No, so we'll consider it a reference name (branch or tag)
                    Err(_) => {
                        info!("using reference name: {}", fragment);
                        (None, Some(fragment))
                    }
                }
            }

            None => (None, None),
        };

        let repository = if self.repository_gix_url.scheme == url::Scheme::File {
            if commit.is_some() || ref_name.is_some() {
                return Err(UrlError::UnsupportedFormat("fragment cannot be used with local git repositories".into()));
            }

            // Use local path
            info!("opening local repository: {}", self.repository_gix_url);

            let path = self.repository_gix_url.path.to_string();
            open(path).map_err(|e| GitError::from(e))?
        } else {
            let (directory, existing) = self.context.cache.directory(&self.repository_url.to_string(), "git-")?;
            let directory = directory.lock().map_err(|e| UrlError::Mutex(e.to_string()))?;

            if existing {
                info!("opening cached repository: {}", directory.to_string_lossy());

                open(directory.clone()).map_err(|e| GitError::from(e))?
            } else {
                info!("cloning repository to: {}", directory.to_string_lossy());

                let mut prepare_fetch = prepare_clone_bare(self.repository_gix_url.clone(), directory.clone())
                    .map_err(|e| GitError::from(e))?
                    .configure_remote(|remote| Ok(remote));

                if commit.is_none() {
                    // Without a specific commit we can get away with a shallow clone
                    let one = NonZeroU32::new(1).unwrap();
                    prepare_fetch = prepare_fetch
                        .with_shallow(remote::fetch::Shallow::DepthAtRemote(one))
                        .with_ref_name(ref_name.as_ref()) // branch or tag (option)
                        .map_err(|e| GitError::from(e))?;
                }

                let (repository, _) = prepare_fetch
                    .fetch_only(progress::Discard, &interrupt::IS_INTERRUPTED)
                    .map_err(|e| GitError::from(e))?;

                repository
            }
        };

        // Note: the entire tree's data will be in memory
        let tree = match commit {
            // Use a specific commit
            Some(commit) => {
                let commit = repository.find_commit(commit).map_err(|e| GitError::from(e))?;
                commit.tree().map_err(|e| GitError::from(e))?
            }

            // Use the HEAD (tip of the branch)
            None => repository.head_tree().map_err(|e| GitError::from(e))?,
        };

        match tree.lookup_entry_by_path(self.path.as_str()).map_err(|e| GitError::from(e))? {
            Some(entry) => {
                // Note: the entire object's data will be in memory,
                // but at least we can "take" it without cloning
                let object = entry.object().map_err(|e| GitError::from(e))?;
                let mut blob = object.try_into_blob().map_err(|e| GitError::from(e))?;
                let data = blob.take_data();

                Ok(Cursor::new(data))
            }

            None => Err(UrlError::new_io_not_found(self)),
        }
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl GitUrl {
    fn conform_path(&mut self) -> Result<(), crate::UrlError> {
        self.path = self.path.normalize();
        Ok(())
    }
}
