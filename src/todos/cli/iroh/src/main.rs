use std::{process, sync::Arc};

use clap::Parser;
use iroh::{blobs::store::Store, node::Node};
use miette::miette;

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
    let storage_path = std::env::current_dir()
        .map_err(|_| miette!("Failed to get pwd"))?
        .join("cdata");
    tokio::fs::create_dir_all(&storage_path)
        .await
        .map_err(|_| miette!("Failed create storage directory"))?;

    // Initialize node
    let node = Node::persistent(&storage_path)
        .await
        .map_err(|_| miette!("Failed to configure persistence Node"))?
        .spawn()
        .await
        .map_err(|_| miette!("Failed to spawn a Node"))?;
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
            output::stdout(&format!("- [ ]: {description}"));
        }
        Some(Command::Toggle { id }) => {
            repo.toggle(id.to_owned()).await?;
            let todo = repo.get(id.to_owned()).await?;
            output::stdout(&format!("- [{}] {}", todo.done_icon(), todo.description));
        }
        Some(Command::Remove { id }) => {
            let todo = repo.get(id.to_owned()).await?;
            repo.remove(id.to_owned()).await?;
            output::stdout(&format!(
                "- [{}] {}. removed",
                todo.done_icon(),
                todo.description
            ));
        }
        None => {
            let todos = repo.list().await?;
            for todo in todos {
                output::stdout(&format!(
                    "- ({}) [{}]: {}",
                    todo.id,
                    todo.done_icon(),
                    &todo.description
                ));
            }
        }
    }
    Ok(())
}
