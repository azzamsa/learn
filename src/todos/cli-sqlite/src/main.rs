#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;

use todos::{cli::Opts, config::Config, exit_codes::ExitCode, output, Error};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {err:?}"));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> miette::Result<ExitCode> {
    let opts = Arc::new(Opts::parse());

    let _config = construct_config(Arc::clone(&opts))?;
    Ok(ExitCode::Success)
}

fn construct_config(_opts: Arc<Opts>) -> Result<Config, crate::Error> {
    Ok(Config {})
}
