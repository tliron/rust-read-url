mod cli;
mod errors;
mod read;
mod run;

use run::*;

use std::process::*;

pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
