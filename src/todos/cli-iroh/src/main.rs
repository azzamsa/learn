#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;
use iroh::node::Node;

use todos::{
    cli::{Command, Opts},
    exit_codes::ExitCode,
    output,
    todo::Repo,
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

    let todo = repo().await?;
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            let id = todo.add(description).await?;
            output::stdout(&format!("- [] {id}: {description}"));
        }
        None => {
            // let todos = todo.list().await?;
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

async fn repo() -> Result<Repo, crate::Error> {
    let node = Node::memory().spawn().await?;
    let client = node.client();
    let doc = client.docs().create().await?;
    let author = client.authors().default().await?;

    Ok(Repo::new(doc, author))
}
