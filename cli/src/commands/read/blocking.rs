use super::super::root::*;

use problemo::*;

impl Root {
    pub fn read_blocking(&self, input_url_or_path: &String) -> Result<(), Problem> {
        use {
            read_url::*,
            std::{fs::*, io},
        };

        let context = UrlContext::new_for(Some(self.cache.clone()));
        let base_urls = context.working_dir_url_vec()?;
        let context = context.with_base_urls(base_urls);

        let url = context.url_or_file_path(input_url_or_path)?;

        tracing::info!("reading from URL (blocking): {}", url);

        let mut reader = io::BufReader::new(url.open()?);

        if self.quiet {
            io::copy(&mut reader, &mut io::sink())?;
        } else {
            match &self.output_path {
                Some(output_path) => {
                    let mut file = io::BufWriter::new(File::create(output_path)?);
                    io::copy(&mut reader, &mut file)?;
                }

                None => {
                    io::copy(&mut reader, &mut io::stdout())?;
                }
            }
        }

        Ok(())
    }
}
