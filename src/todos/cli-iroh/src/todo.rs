use std::{fmt, str::FromStr};

use iroh::{
    base::base32,
    blobs::store::fs::Store,
    client::{docs::Entry, MemDoc as Doc},
    docs::{AuthorId, NamespaceId},
    node::Node,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json as json;
use tokio_stream::StreamExt;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

pub struct Repo {
    pub document: Doc,
    pub author_id: AuthorId,
}

impl Repo {
    pub fn new(document: Doc, author_id: AuthorId) -> Self {
        Self {
            document,
            author_id,
        }
    }
    // pub async fn list(&self) -> Result<Vec<Todo>, crate::Error> {
    //     let mut entries = self.document.get_many(Query::all()).await?;
    //
    //     let mut todos = Vec::new();
    //     while let Some(entry) = entries.next().await {
    //         let entry = entry?;
    //         let todo = self.todo_from_entry(&entry).await?;
    //         todos.push(todo);
    //     }
    //
    //     dbg!("todos: {:?}", &todos);
    //     Ok(todos)
    // }
    pub async fn add(&self, description: &str) -> Result<i32, crate::Error> {
        let id = self.id();
        let todo = Todo {
            id,
            description: description.to_string(),
            done: false,
        };
        self.write(id.to_string(), todo.to_string()).await?;
        Ok(id)
    }
    async fn todo_from_entry(&self, entry: &Entry) -> Result<Todo, crate::Error> {
        let hash = entry.content_hash();
        let hash = base32::fmt_short(hash.as_bytes());
        Todo::from_str(&hash)
    }
    fn id(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=90)
    }
    async fn write(&self, key: String, content: String) -> Result<(), crate::Error> {
        self.document
            .set_bytes(self.author_id, key, content)
            .await?;
        Ok(())
    }
}

impl Todo {
    fn from_str(str: &str) -> Result<Self, crate::Error> {
        let todo = json::from_str(str)?;
        Ok(todo)
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl Repo {
    pub async fn read(node: &Node<Store>, config: Config) -> Result<Repo, crate::Error> {
        let author_id = Self::get_author(node).await?;
        let document = node
            .client()
            .docs()
            .open(NamespaceId::from_str(&config.document_id)?)
            .await?;
        let document = document.expect("can't find document");
        let document_id = &document.id().to_string();

        println!(
            r#"Loaded.
document id: {}
author   id: {}"#,
            document_id, author_id
        );
        Ok(Repo {
            document,
            author_id,
        })
    }
    pub async fn init(node: &Node<Store>) -> Result<Repo, crate::Error> {
        let author_id = Self::get_author(node).await?;

        tokio::fs::create_dir_all(Config::storage_path()).await?;
        let document = node.client().docs().create().await?;
        let document_id = &document.id().to_string();
        let config = Config::new(document_id, &author_id.to_string());
        config.write()?;

        println!(
            r#"Initialized.
document id: {}
author   id: {}"#,
            document_id, author_id
        );
        Ok(Repo {
            document,
            author_id,
        })
    }
    async fn get_author(
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
}
