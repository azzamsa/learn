#![deny(unsafe_code)]
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::{process, sync::Arc};

use clap::Parser;
use iroh::docs::{AuthorId, NamespaceId};
use iroh::node::Node;
use tokio_stream::StreamExt;

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

    let repo = init_repo().await?;
    match opts.cmd.as_ref() {
        Some(Command::Add { description }) => {
            let id = repo.add(description).await?;
            output::stdout(&format!("- [] {id}: {description}"));
        }
        None => {
            let todos = repo.list().await?;
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

async fn init_repo() -> Result<Repo, crate::Error> {
    let storage_path = Path::new(env!("HOME"))
        .join(".local")
        .join("share")
        .join("todos-iroh");
    let config_path = storage_path.join("todos.cfg");

    // Initialize node
    let node = Node::persistent(&storage_path).await?.spawn().await?;
    let author_id = get_author(&node).await?;
    dbg!("author id: {}", &author_id);

    // Check if the previous document_id exists
    let document = match config_path.exists() {
        true => {
            println!("Using previous config.");
            let document_id = fs::read_to_string(config_path)?;
            let document = node
                .client()
                .docs()
                .open(NamespaceId::from_str(&document_id)?)
                .await?;
            document.expect("can't find document")
        }
        false => {
            println!("Creating new config.");
            tokio::fs::create_dir_all(&storage_path).await?;
            let document = node.client().docs().create().await?;
            let document_id = document.id().to_string();
            fs::write(config_path, document_id)?;
            document
        }
    };

    dbg!("document: {}", &document.id().to_string());
    Ok(Repo {
        document,
        author_id,
    })
}

pub async fn get_author(
    node: &Node<iroh::blobs::store::fs::Store>,
) -> Result<AuthorId, crate::Error> {
    let mut stream = node.client().authors().list().await?;
    if let Some(author_id) = stream.next().await {
        let author_id = author_id?;
        return Ok(author_id);
    }
    let author_id = node.client().authors().create().await?;
    Ok(author_id)
}
