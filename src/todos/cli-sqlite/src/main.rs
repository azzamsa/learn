#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;

use todos::{cli::Opts, config::Config, db, exit_codes::ExitCode, output, Error};

#[tokio::main]
async fn main() {
    let result = run().await;
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

async fn run() -> miette::Result<ExitCode> {
    let opts = Arc::new(Opts::parse());

    let _config = construct_config(Arc::clone(&opts))?;

    let db = db::connect().await?;
    db::migrate(db).await?;
    Ok(ExitCode::Success)
}

fn construct_config(_opts: Arc<Opts>) -> Result<Config, crate::Error> {
    Ok(Config {})
}
