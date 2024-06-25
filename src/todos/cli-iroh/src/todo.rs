use bytes::Bytes;
use iroh::base::base32;
use iroh::{
    client::{docs::Entry, MemDoc as Doc},
    docs::{store::Query, AuthorId},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json as json;
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize)]
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
    pub async fn list(&self) -> Result<Vec<Todo>, crate::Error> {
        let mut entries = self.document.get_many(Query::all()).await?;

        let mut todos = Vec::new();
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let todo = self.todo_from_entry(&entry).await?;
            todos.push(todo);
        }

        Ok(todos)
    }
    pub async fn add(&self, description: &str) -> Result<i32, crate::Error> {
        let id = self.id();
        let todo = Todo {
            id,
            description: description.to_string(),
            done: false,
        };
        self.insert_bytes(id, todo.as_bytes()?).await?;
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
    async fn insert_bytes(&self, key: i32, content: Bytes) -> Result<(), crate::Error> {
        let key: Bytes = Bytes::copy_from_slice(&key.to_le_bytes());
        self.document
            .set_bytes(self.author_id, key, content)
            .await?;
        Ok(())
    }
}

impl Todo {
    fn as_bytes(&self) -> Result<Bytes, crate::Error> {
        let buf = json::to_vec(self)?;
        Ok(buf.into())
    }
    fn from_str(str: &str) -> Result<Self, crate::Error> {
        let todo = json::from_str(str)?;
        Ok(todo)
    }
}
