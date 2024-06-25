#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;

use todos::{
    cli::{Command, Opts},
    config::Config,
    db,
    exit_codes::ExitCode,
    output,
    todo::Repo,
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

    let todo = Repo::new(pool);
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            let id = todo.add(description).await?;
            output::stdout(&format!("- [] {id}: {description}"));
        }
        Some(Command::Mark { id }) => {
            todo.mark(*id).await?;
            let description = todo.description(*id).await?;
            output::stdout(&format!("- [X] {id}: {description}"));
        }
        Some(Command::Unmark { id }) => {
            todo.unmark(*id).await?;
            let description = todo.description(*id).await?;
            output::stdout(&format!("- [] {id}: {description}"));
        }
        Some(Command::Remove { id }) => {
            let description = todo.description(*id).await?;
            let status = todo.status(*id).await?;
            todo.remove(*id).await?;
            let status = match status {
                true => "X",
                false => "",
            };
            output::stdout(&format!("- [{status}] {id}: {description}.  removed"));
        }
        None => {
            let todos = todo.list().await?;
            for todo in todos {
                output::stdout(&format!(
                    "- [{}] {}: {}",
                    if todo.done { "X" } else { "" },
                    todo.id,
                    &todo.description,
                ));
            }
        }
    }
    Ok(ExitCode::Success)
}
