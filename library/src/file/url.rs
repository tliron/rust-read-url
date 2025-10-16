use super::{
    super::{context::*, url::*, util::*},
    file_url::*,
};

use {kutil::std::error::*, std::path::*};

impl URL for FileUrl {
    fn context(&self) -> &UrlContext {
        &*self.context
    }

    fn query(&self) -> Option<UrlQuery> {
        self.query.clone()
    }

    fn fragment(&self) -> Option<String> {
        self.fragment.clone()
    }

    fn format(&self) -> Option<String> {
        get_format_from_path(&self.path.to_string_lossy())
    }

    fn local(&self) -> Option<PathBuf> {
        Some(self.path.clone())
    }

    fn base(&self) -> Option<UrlRef> {
        self.path.parent().map(|path| {
            let mut path = path.to_string_lossy();
            path += MAIN_SEPARATOR_STR;
            self.new_with(path.to_string().into()).into()
        })
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

        async fn conform_async(mut url: FileUrl) -> Result<UrlRef, Problem> {
            url.conform_path()?;
            Ok(url.into())
        }

        Ok(Box::pin(conform_async(self.clone())))
    }

    #[cfg(feature = "blocking")]
    fn open(&self) -> Result<ReadRef, problemo::Problem> {
        use std::fs::*;

        Ok(Box::new(File::open(&self.path).map_err(|error| self.into_problem(error))?))
    }

    #[cfg(feature = "async")]
    fn open_async(&self) -> Result<OpenFuture, problemo::Problem> {
        use {problemo::*, tokio::fs::*};

        async fn open_async(url: FileUrl) -> Result<AsyncReadRef, Problem> {
            let file = File::open(&url.path).await.map_err(|error| url.into_problem(error))?;
            Ok(Box::pin(file))
        }

        Ok(Box::pin(open_async(self.clone())))
    }
}

#[cfg(any(feature = "blocking", feature = "async"))]
impl FileUrl {
    fn conform_path(&mut self) -> Result<(), problemo::Problem> {
        self.path = match conform_file_path(&self.path) {
            Ok(path) => path,
            Err(error) => {
                return Err(self.into_problem(error));
            }
        };
        Ok(())
    }

    fn into_problem(&self, error: std::io::Error) -> problemo::Problem {
        use {
            super::super::errors::*,
            problemo::{common::*, *},
            std::io,
        };

        let error = error.with_path(&self.path);
        match error.kind() {
            io::ErrorKind::NotFound => error.into_problem().via(LowLevelError).via(UnreachableError::new("URL")),
            _ => error.into_problem().via(LowLevelError),
        }
        .via(UrlError)
        .with(SchemeAttachment::new("file"))
    }
}
