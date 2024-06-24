#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;

use todos::{
    cli::{Command, Opts},
    config::Config,
    db,
    exit_codes::ExitCode,
    output,
    todo::Todo,
    Error,
};

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

    let pool = db::connect().await?;
    db::migrate(&pool).await?;

    let todo = Todo::new(pool);
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            todo.add(description).await?;
            println!("- [] {description}");
        }
        Some(Command::Mark { id }) => {
            let description = todo.mark(*id).await?;
            println!("- [X] {id}: {description}");
        }
        Some(Command::Unmark { id }) => {
            let description = todo.unmark(*id).await?;
            println!("- [] {id}: {description}");
        }
        None => {
            todo.list().await?;
        }
    }
    Ok(ExitCode::Success)
}

fn construct_config(_opts: Arc<Opts>) -> Result<Config, crate::Error> {
    Ok(Config {})
}
