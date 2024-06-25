#![deny(unsafe_code)]
use std::path::Path;
use std::str::FromStr;
use std::{process, sync::Arc};

use clap::Parser;
use iroh::docs::NamespaceId;
use iroh::node::Node;

use todos::{
    cli::{Command, Opts},
    config,
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
    let path = Path::new(env!("HOME"))
        .join(".local")
        .join("share")
        .join("todos-iroh");

    // https://docs.rs/iroh/latest/iroh/node/struct.Node.html
    let node = Node::persistent(&path).await?.spawn().await?;
    // https://docs.rs/iroh/latest/iroh/client/type.MemIroh.html
    let client = node.client();

    let (doc, author) = match config::load(&path) {
        Ok(c) => {
            // https://docs.rs/iroh/latest/iroh/client/docs/struct.Client.html#method.open
            // https://docs.rs/iroh/latest/iroh/docs/struct.NamespaceId.html
            let doc = client
                .docs()
                .open(NamespaceId::from_str(&c.document)?)
                .await?;
            let author = client.authors().default().await?;
            (doc, author)
        }
        Err(_) => {
            tokio::fs::create_dir_all(&path).await?;
            // https://docs.rs/iroh/latest/iroh/client/docs/struct.Client.html
            // https://docs.rs/iroh/latest/iroh/client/docs/struct.Doc.html
            let doc = client.docs().create().await?;
            // https://docs.rs/iroh/latest/iroh/client/struct.Iroh.html#method.authors
            // https://docs.rs/iroh/latest/iroh/client/authors/struct.Client.html#method.default
            // https://docs.rs/iroh/latest/iroh/docs/struct.AuthorId.html
            let author = client.authors().default().await?;
            (Some(doc), author)
        }
    };

    let doc = doc.expect("a");
    // https://docs.rs/iroh/latest/iroh/docs/struct.NamespaceId.html
    let doc_id = doc.id().fmt_short();
    let author_id = author.fmt_short();
    let config = config::Config {
        document: doc_id,
        author: author_id,
    };
    config::save(&path, &config)?;

    Ok(Repo::new(doc, author))
}
