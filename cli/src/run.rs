use super::{cli::*, errors::*};

use {clap::*, kutil_cli::log::*};

pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    if !cli.quiet {
        cli.colorize.initialize();
        initialize_tracing(cli.verbose + 2, cli.log_path.as_ref())?;
    }

    match &cli.subcommand {
        None => cli.read()?,
        Some(subcommand) => match subcommand {
            SubCommand::Version(version) => version.run::<CLI>(),
            SubCommand::Completion(completion) => completion.run::<CLI>(),
        },
    }

    Ok(())
}
