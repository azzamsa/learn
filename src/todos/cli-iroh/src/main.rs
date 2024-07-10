#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;
use iroh::node::Node;

use todos::{
    cli::{Command, Opts},
    config::Config,
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

    // Initialize node
    let node = Node::persistent(Config::storage_path())
        .await
        .unwrap()
        .spawn()
        .await
        .unwrap();
    let repo = match Config::read() {
        Ok(config) => {
            println!("Using previous config.");
            Repo::read(&node, config).await.unwrap()
        }
        Err(_) => Repo::init(&node).await.unwrap(),
    };

    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            let id = repo.add(description).await?;
            output::stdout(&format!("- [] {id}: {description}"));
        }
        None => {
            // let todos = repo.list().await?;
            // for todo in todos {
            //     output::stdout(&format!(
            //         "- [{}] {}: {}",
            //         if todo.done { "X" } else { "" },
            //         todo.id,
            //         &todo.description,
            //     ));
            // }
        }
    }
    Ok(ExitCode::Success)
}
