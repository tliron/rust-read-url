use super::{cli::*, errors::*};

impl CLI {
    /// Read.
    pub fn read(&self) -> Result<(), MainError> {
        if self.asynchronous {
            #[cfg(feature = "async")]
            {
                use tokio::runtime;

                let runtime = runtime::Runtime::new()?;
                runtime.block_on(self.read_async())
            }

            #[cfg(not(feature = "async"))]
            Err(kutil::cli::run::Exit::new(1, Some("\"async\" feature not enabled during compilation")).into())
        } else {
            #[cfg(feature = "blocking")]
            {
                self.read_blocking()
            }

            #[cfg(not(feature = "blocking"))]
            Err(kutil::cli::run::Exit::new(1, Some("\"blocking\" feature not enabled during compilation")).into())
        }
    }

    #[cfg(feature = "blocking")]
    fn read_blocking(&self) -> Result<(), MainError> {
        use {
            read_url::*,
            std::{fs::*, io},
        };

        let context = UrlContext::new_for(Some(self.cache.clone()));
        let base_urls = context.working_dir_url_vec()?;
        let context = context.with_base_urls(base_urls);

        let url = context.url_or_file_path(&self.input_url_or_path)?;

        tracing::info!("reading from URL (blocking): {}", url);

        let mut reader = url.open()?;
        if self.quiet {
            io::copy(&mut reader, &mut io::sink())?;
        } else {
            match &self.output_path {
                Some(output_path) => {
                    let mut file = File::create(output_path)?;
                    io::copy(&mut reader, &mut file)?;
                }

                None => {
                    io::copy(&mut reader, &mut io::stdout())?;
                }
            }
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    async fn read_async(&self) -> Result<(), MainError> {
        use {
            read_url::*,
            tokio::{fs::*, io},
        };

        let context = UrlContext::new_for(Some(self.cache.clone()));
        let base_urls = context.working_dir_url_vec()?;
        let context = context.with_base_urls(base_urls);

        let url = context.url_or_file_path_async(&self.input_url_or_path).await?;

        tracing::info!("reading from URL (asynchronous): {}", url);

        let mut reader = url.open_async()?.await?;
        if self.quiet {
            io::copy(&mut reader, &mut io::sink()).await?;
        } else {
            match &self.output_path {
                Some(output_path) => {
                    let mut file = File::create(output_path).await?;
                    io::copy(&mut reader, &mut file).await?;
                }

                None => {
                    io::copy(&mut reader, &mut io::stdout()).await?;
                }
            }
        }

        Ok(())
    }
}
