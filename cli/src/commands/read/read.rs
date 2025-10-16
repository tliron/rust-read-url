use super::super::{super::errors::*, root::*};

impl Root {
    /// Read.
    #[allow(unused_variables)]
    pub fn read(&self) -> Result<(), MainError> {
        let Some(input_url_or_path) = &self.input_url_or_path else {
            return Err(MainError::Missing("input URL or path".into()));
        };

        if self.asynchronous {
            #[cfg(feature = "async")]
            {
                use tokio::runtime;

                let runtime = runtime::Runtime::new()?;
                runtime.block_on(self.read_async(input_url_or_path))
            }

            #[cfg(not(feature = "async"))]
            Err(kutil::cli::run::Exit::new(1, Some("\"async\" feature not enabled during compilation")).into())
        } else {
            #[cfg(feature = "blocking")]
            {
                self.read_blocking(input_url_or_path)
            }

            #[cfg(not(feature = "blocking"))]
            Err(kutil::cli::run::Exit::new(
                1,
                Some("\"blocking\" feature not enabled during compilation, must use --async"),
            )
            .into())
        }
    }
}
