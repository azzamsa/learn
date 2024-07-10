use std::fmt;
use std::str::FromStr;

use iroh::{
    client::{docs::Entry, Doc, Iroh},
    docs::{store::Query, AuthorId, Capability, NamespaceSecret},
};
use serde::{Deserialize, Serialize};
use serde_json as json;
use tokio_stream::StreamExt;
use uuid::Uuid;

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
    pub async fn add(&self, description: &str) -> Result<(), crate::Error> {
        let id = Self::gen_id();
        let created = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let todo = Todo {
            created,
            id,
            description: description.to_string(),
            done: false,
        };
        self.insert(id.to_string(), todo.to_string()).await?;
        Ok(())
    }
    pub async fn list(&self) -> Result<Vec<Todo>, crate::Error> {
        let mut entries = self
            .document
            .get_many(Query::single_latest_per_key())
            .await?;

        let mut todos = Vec::new();
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let todo = self.todo_from_entry(&entry).await?;
            todos.push(todo);
        }
        todos.sort_by_key(|t| t.created);
        Ok(todos)
    }
    pub async fn toggle_done(&mut self, id: String) -> Result<(), crate::Error> {
        let mut todo = self.get_todo(id.clone()).await?;
        todo.done = !todo.done;
        self.insert(id, todo.to_string()).await
    }
    async fn get_todo(&self, id: String) -> Result<Todo, crate::Error> {
        let entry = self
            .document
            .get_many(Query::single_latest_per_key().key_exact(id))
            .await?
            .next()
            .await
            .ok_or_else(|| crate::Error::Internal("Task not found".into()))??;

        self.todo_from_entry(&entry).await
    }
    async fn todo_from_entry(&self, entry: &Entry) -> Result<Todo, crate::Error> {
        let content = entry.content_bytes(&self.document).await?;
        let content = std::str::from_utf8(&content)?;
        dbg!(&content);
        Todo::from_str(content)
    }
    fn gen_id() -> Uuid {
        Uuid::new_v4()
    }
    async fn insert(&self, key: String, content: String) -> Result<(), crate::Error> {
        self.document
            .set_bytes(self.author_id, key, content)
            .await?;
        Ok(())
    }
}

impl Repo {
    pub async fn read(client: &Iroh, namespace_id: &str) -> Result<Repo, crate::Error> {
        let author_id = client.authors().default().await?;
        let document = Self::load_document(client, namespace_id).await?;

        tracing::debug!(
            r#"Loaded.
document id: {}
author   id: {}"#,
            document.id(),
            author_id,
        );
        Ok(Repo {
            document,
            author_id,
        })
    }
    async fn load_document(client: &Iroh, namespace_id: &str) -> Result<Doc, crate::Error> {
        let document = client
            .docs()
            .import_namespace(Capability::Write(NamespaceSecret::from_str(namespace_id)?))
            .await?;
        Ok(document)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    /// Record creation timestamp. Counted as micros since the Unix epoch.
    pub created: u64,
    /// ID of the todo
    pub id: Uuid,
    /// Description of the todo
    pub description: String,
    /// Whether or not the todo has been completed.
    pub done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl Todo {
    // pub fn to_string(&self) -> String {
    //     serde_json::to_string(self).unwrap()
    // }
    fn from_str(str: &str) -> Result<Self, crate::Error> {
        let todo = json::from_str(str)?;
        Ok(todo)
    }
}
