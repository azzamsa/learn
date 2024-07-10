#![deny(unsafe_code)]
use std::{process, sync::Arc};

use clap::Parser;
use iroh::{blobs::store::Store, node::Node};

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
    let storage_path = std::env::current_dir().unwrap().join("cdata");
    tokio::fs::create_dir_all(&storage_path).await.unwrap();

    // Initialize node
    let node = Node::persistent(&storage_path)
        .await
        .unwrap()
        .spawn()
        .await
        .unwrap();
    run_inner(&node).await?;
    // Shutdown the node to make sure all writes are flushed.
    if let Err(err) = node.shutdown().await {
        println!("Error during shutdown: {err:?}");
    }
    Ok(ExitCode::Success)
}

async fn run_inner<D: Store>(node: &Node<D>) -> Result<(), crate::Error> {
    let opts = Arc::new(Opts::parse());

    let namespace_id = "q4hiommvh3ttec3x2y7h4le5tkx2tee762s6miu4rer6d2asi4la";
    let client = node.client();

    let repo = Repo::read(client, namespace_id).await?;
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            repo.add(description).await?;
            output::stdout(&format!("- [] {description}"));
        }
        None => {
            let todos = repo.list().await?;
            for todo in todos {
                output::stdout(&format!(
                    "- [{}] {}",
                    if todo.done { "X" } else { "" },
                    &todo.description,
                ));
            }
        }
    }
    Ok(())
}
