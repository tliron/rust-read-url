use super::super::{super::errors::*, root::*};

impl Root {
    pub async fn read_async(&self, input_url_or_path: &String) -> Result<(), MainError> {
        use {
            read_url::*,
            tokio::{fs::*, io},
        };

        let context = UrlContext::new_for(Some(self.cache.clone()));
        let base_urls = context.working_dir_url_vec()?;
        let context = context.with_base_urls(base_urls);

        let url = context.url_or_file_path_async(input_url_or_path).await?;

        tracing::info!("reading from URL (asynchronous): {}", url);

        let mut reader = io::BufReader::new(url.open_async()?.await?);

        if self.quiet {
            io::copy(&mut reader, &mut io::sink()).await?;
        } else {
            match &self.output_path {
                Some(output_path) => {
                    let mut file = io::BufWriter::new(File::create(output_path).await?);
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
