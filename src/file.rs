use super::{context::*, errors::*, url::*};

use {
    path_absolutize::*,
    std::{env::current_dir, fmt, fs::File, io, path::*},
};

//
// FileUrl
//

#[derive(Debug, Clone)]
pub struct FileUrl {
    pub context: ContextRef,
    pub path: Box<Path>,
}

impl Context {
    pub fn new_file_url(self: &ContextRef, path: &Path) -> FileUrl {
        FileUrl { context: self.clone(), path: path.into() }
    }

    pub fn new_valid_file_url(self: &ContextRef, path: &Path) -> Result<FileUrl, UrlError> {
        match path.try_exists()? {
            true => match path.absolutize() {
                Ok(path) => {
                    let mut path = path.to_path_buf();

                    if path.is_dir() {
                        let mut path_string = path.into_os_string();
                        path_string.push(MAIN_SEPARATOR_STR);
                        path = path_string.into();
                    }

                    Ok(self.new_file_url(&path))
                }

                Err(err) => Err(err.into()),
            },

            false => Err(UrlError::new_not_found(&*path.to_string_lossy())),
        }
    }

    pub fn new_working_dir_url(self: &ContextRef) -> Result<FileUrl, UrlError> {
        self.new_valid_file_url(&current_dir()?)
    }

    pub fn new_working_dir_url_vec(self: &ContextRef) -> Result<Vec<UrlRef>, UrlError> {
        let url = self.new_working_dir_url()?;
        Ok(vec![url.into()])
    }
}

impl FileUrl {
    pub fn is_dir(&self) -> bool {
        self.path.ends_with(MAIN_SEPARATOR_STR)
    }
}

impl Url for FileUrl {
    fn context(&self) -> &Context {
        &*self.context
    }

    fn format(&self) -> Option<String> {
        match self.path.extension() {
            Some(extension) => Some(extension.to_string_lossy().into()),
            None => None,
        }
    }

    fn base(&self) -> Option<UrlRef> {
        match self.path.parent() {
            Some(path) => {
                let mut path = path.to_path_buf().into_os_string();
                path.push(MAIN_SEPARATOR_STR);

                let url = self.context.new_file_url(Path::new(&path));
                Some(url.into())
            }

            None => None,
        }
    }

    fn relative(&self, path: &str) -> UrlRef {
        let path = self.path.join(path);
        let url = self.context.new_file_url(&path);
        url.into()
    }

    fn valid_relative(&self, path: &str) -> Result<UrlRef, UrlError> {
        let path = self.path.join(path);
        let url = self.context.new_valid_file_url(&path)?;
        Ok(url.into())
    }

    fn key(&self) -> String {
        format!("{}", self)
    }

    fn open(&self) -> Result<Box<dyn io::Read>, UrlError> {
        Ok(Box::new(File::open(self.path.clone())?))
    }
}

impl fmt::Display for FileUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "file://{}", self.path.display())
    }
}

impl Into<UrlRef> for FileUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
