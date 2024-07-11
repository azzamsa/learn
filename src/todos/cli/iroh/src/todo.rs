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
    pub async fn add(&self, description: &str) -> Result<Todo, crate::Error> {
        let id = Self::gen_id();
        let created = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let todo = Todo {
            created,
            id: id.clone(),
            description: description.to_string(),
            done: false,
        };
        self.insert(id.clone(), todo.to_string()).await?;

        self.get(id).await
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
    pub async fn toggle(&self, id: String) -> Result<Todo, crate::Error> {
        let mut todo = self.get(id.clone()).await?;
        todo.done = !todo.done;
        self.insert(id.clone(), todo.to_string()).await?;

        self.get(id).await
    }
    pub async fn remove(self, id: String) -> Result<Todo, crate::Error> {
        let todo = self.get(id.clone()).await?;
        self.delete(id).await?;
        Ok(todo)
    }
    async fn get(&self, id: String) -> Result<Todo, crate::Error> {
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
        Todo::from_str(content)
    }
    fn gen_id() -> String {
        // Uuid is too long to type
        let uuid = Uuid::new_v4();
        // Get the first 8 characters (32 bits) of the UUID as a hexadecimal string
        uuid.as_simple()
            .to_string()
            .chars()
            .take(8)
            .collect::<String>()
    }
    async fn insert(&self, key: String, content: String) -> Result<(), crate::Error> {
        self.document
            .set_bytes(self.author_id, key, content)
            .await?;
        Ok(())
    }
    async fn delete(&self, key: String) -> Result<(), crate::Error> {
        self.document.del(self.author_id, key).await?;
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
    pub id: String,
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
    fn from_str(str: &str) -> Result<Self, crate::Error> {
        let todo = json::from_str(str)?;
        Ok(todo)
    }
    pub fn done_icon(&self) -> String {
        if self.done {
            "X".to_string()
        } else {
            " ".to_string()
        }
    }
}
