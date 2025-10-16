use super::super::root::*;

use problemo::{common::*, *};

impl Root {
    /// Read.
    #[allow(unused_variables)]
    pub fn read(&self) -> Result<(), Problem> {
        let Some(input_url_or_path) = &self.input_url_or_path else {
            return Err(MissingError::new("input URL or path").into());
        };

        if self.asynchronous {
            #[cfg(feature = "async")]
            {
                use tokio::runtime;

                let runtime = runtime::Runtime::new()?;
                runtime.block_on(self.read_async(input_url_or_path))
            }

            #[cfg(not(feature = "async"))]
            Err(Problem!("\"async\" feature not enabled during compilation"))
        } else {
            #[cfg(feature = "blocking")]
            {
                self.read_blocking(input_url_or_path)
            }

            #[cfg(not(feature = "blocking"))]
            Err(Problem!("\"blocking\" feature not enabled during compilation, must use --async"))
        }
    }
}
