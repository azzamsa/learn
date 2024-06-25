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
    let config = Arc::new(Config::load()?);

    let pool = db::connect(&config.database).await?;
    db::migrate(&pool).await?;

    let todo = Todo::new(pool);
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            let id = todo.add(description).await?;
            println!("- [] {id}: {description}");
        }
        Some(Command::Mark { id }) => {
            todo.mark(*id).await?;
            let description = todo.description(*id).await?;
            println!("- [X] {id}: {description}");
        }
        Some(Command::Unmark { id }) => {
            todo.unmark(*id).await?;
            let description = todo.description(*id).await?;
            println!("- [] {id}: {description}");
        }
        Some(Command::Remove { id }) => {
            let description = todo.description(*id).await?;
            let status = todo.status(*id).await?;
            todo.remove(*id).await?;
            let status = match status {
                true => "X",
                false => "",
            };
            println!("- [{status}] {id}: {description}.  removed");
        }
        None => {
            todo.list().await?;
        }
    }
    Ok(ExitCode::Success)
}
