use super::{
    super::{context::*, url::*, util::*},
    git_url::*,
};

impl URL for GitUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<UrlQuery> {
        self.repository_url.query()
    }

    fn fragment(&self) -> Option<String> {
        self.repository_url.fragment()
    }

    fn format(&self) -> Option<String> {
        get_format_from_path(&self.path)
    }

    fn base(&self) -> Option<UrlRef> {
        get_relative_path_parent(&self.path).map(|path| self.new_with(path).into())
    }

    fn relative(&self, path: &str) -> UrlRef {
        self.new_with(self.path.join(path)).into()
    }

    #[cfg(feature = "blocking")]
    fn conform(&mut self) -> Result<(), problemo::Problem> {
        self.conform_path()
    }

    #[cfg(feature = "async")]
    fn conform_async(&self) -> Result<ConformFuture, problemo::Problem> {
        use problemo::*;

        async fn conform_async(mut url: GitUrl) -> Result<UrlRef, Problem> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, problemo::Problem> {
        Ok(Box::new(self.open_cursor()?))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, problemo::Problem> {
        use problemo::*;

        async fn open_async(url: GitUrl) -> Result<AsyncReadRef, Problem> {
            Ok(Box::pin(url.open_cursor()?))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl GitUrl {
    fn open_cursor(&self) -> Result<std::io::Cursor<Vec<u8>>, problemo::Problem> {
        use {
            super::super::errors::*,
            gix::*,
            problemo::common::*,
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
                return Err(InvalidError::as_problem("fragment cannot be used with local git repositories")
                    .via(UrlError)
                    .with(SchemeAttachment::new("git")));
            }

            // Use local path
            info!("opening local repository: {}", self.repository_gix_url);

            let path = self.repository_gix_url.path.to_string();
            open(path).into_url_problem("git")?
        } else {
            let (directory, existing) = self.context.cache.directory(&self.repository_url.to_string(), "git-")?;
            let directory = directory.lock().into_thread_problem()?;

            if existing {
                info!("opening cached repository: {}", directory.display());

                open(directory.clone()).into_url_problem("git")?
            } else {
                info!("cloning repository to: {}", directory.display());

                let mut prepare_fetch =
                    prepare_clone_bare(self.repository_gix_url.clone(), directory.clone()).into_url_problem("git")?;

                if commit.is_none() {
                    // Without a specific commit we can get away with a shallow clone
                    let one = NonZeroU32::new(1).expect("NonZeroU32::new");
                    prepare_fetch = prepare_fetch
                        .with_shallow(remote::fetch::Shallow::DepthAtRemote(one))
                        .with_ref_name(ref_name.as_ref()) // branch or tag (option)
                        .into_url_problem("git")?;
                }

                let (repository, _) =
                    prepare_fetch.fetch_only(progress::Discard, &interrupt::IS_INTERRUPTED).into_url_problem("git")?;
                repository
            }
        };

        // Note: the entire tree's data will be in memory
        let tree = match commit {
            // Use a specific commit
            Some(commit) => {
                let commit = repository.find_commit(commit).into_url_problem("git")?;
                commit.tree().into_url_problem("git")?
            }

            // Use the HEAD (tip of the branch)
            None => repository.head_tree().into_url_problem("git")?,
        };

        let entry = tree
            .lookup_entry_by_path(self.path.as_str())
            .into_url_problem("git")?
            .ok_or_else(|| unreachable_url(self, "git"))?;

        // Note: the entire object's data will be in memory,
        // but at least we can "take" it without cloning
        let object = entry.object().into_url_problem("git")?;
        let mut blob = object.try_into_blob().into_url_problem("git")?;
        let data = blob.take_data();

        Ok(Cursor::new(data))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl GitUrl {
    fn conform_path(&mut self) -> Result<(), problemo::Problem> {
        self.path = self.path.normalize();
        Ok(())
    }
}
