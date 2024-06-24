#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;

use todos::{
    cli::{Command, Opts},
    config::Config,
    db,
    exit_codes::ExitCode,
    output, todo, Error,
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

    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{description}'");
            let todo_id = todo::add(&pool, description).await?;
            println!("Added new todo with id {todo_id}");
        }
        Some(Command::Mark { id }) => {
            println!("Marking todo {id} as done");
        }
        Some(Command::Unmark { id }) => {
            println!("Marking todo {id} as done");
        }
        None => {
            println!("Printing list of all todos");
        }
    }
    Ok(ExitCode::Success)
}

fn construct_config(_opts: Arc<Opts>) -> Result<Config, crate::Error> {
    Ok(Config {})
}
