use super::{context::*, errors::*, url::*};

use std::{fmt, io, path::*};

//
// HttpUrl
//

/// HTTP URL.
#[derive(Debug, Clone)]
pub struct HttpUrl {
    /// The [Url](url::Url).
    pub url: url::Url,

    context: ContextRef,
}

impl Context {
    /// Constructor.
    pub fn new_http_url(self: &ContextRef, url: url::Url) -> HttpUrl {
        HttpUrl { context: self.clone(), url }
    }

    /// Constructor.
    pub fn new_valid_http_url(self: &ContextRef, url: url::Url) -> Result<HttpUrl, UrlError> {
        let response = self.http_client.head(url.clone()).send()?;
        if response.status().is_success() {
            Ok(self.new_http_url(url))
        } else {
            Err(UrlError::new_not_found(url.as_str()))
        }
    }

    /// Constructor.
    pub fn new_http_url_from_string(self: &ContextRef, url: &str) -> Result<HttpUrl, UrlError> {
        let url = url::Url::parse(url)?;
        Ok(self.new_http_url(url))
    }

    /// Constructor.
    pub fn new_valid_http_url_from_string(self: &ContextRef, url: &str) -> Result<HttpUrl, UrlError> {
        let url = url::Url::parse(url)?;
        self.new_valid_http_url(url)
    }
}

impl URL for HttpUrl {
    fn context(&self) -> &Context {
        &*self.context
    }

    fn format(&self) -> Option<String> {
        match Path::new(self.url.path()).extension() {
            Some(extension) => Some(extension.to_string_lossy().into()),
            None => None,
        }
    }

    fn base(&self) -> Option<UrlRef> {
        match Path::new(self.url.path()).parent() {
            Some(path) => {
                let mut url = self.url.clone();
                url.set_path(&*path.to_string_lossy());
                let url = self.context.new_http_url(url);
                Some(url.into())
            }

            None => None,
        }
    }

    fn relative(&self, path: &str) -> UrlRef {
        let path = Path::new(self.url.path()).join(path);
        let mut url = self.url.clone();
        url.set_path(&*path.to_string_lossy());
        let url = self.context.new_http_url(url);
        url.into()
    }

    fn valid_relative(&self, path: &str) -> Result<UrlRef, UrlError> {
        let path = Path::new(self.url.path()).join(path);
        let mut url = self.url.clone();
        url.set_path(&*path.to_string_lossy());
        let url = self.context.new_valid_http_url(url)?;
        Ok(url.into())
    }

    fn key(&self) -> String {
        format!("{}", self)
    }

    fn open(&self) -> Result<Box<dyn io::Read>, UrlError> {
        let response = self.context.http_client.get(self.url.clone()).send()?;
        Ok(Box::new(response))
    }
}

impl fmt::Display for HttpUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.url)
    }
}

impl Into<UrlRef> for HttpUrl {
    fn into(self) -> UrlRef {
        Box::new(self)
    }
}
