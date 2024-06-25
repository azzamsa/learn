use bytes::Bytes;
use iroh::{client::MemDoc, docs::AuthorId};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

pub struct Repo {
    doc: MemDoc,
    author: AuthorId,
}

impl Repo {
    pub fn new(doc: MemDoc, author: AuthorId) -> Self {
        Self { doc, author }
    }
    pub async fn add(&self, description: &str) -> Result<i32, crate::Error> {
        let id = self.id().await;
        let todo = Todo {
            id,
            description: description.to_string(),
            done: false,
        };
        self.insert_bytes(self.id().await, todo.as_bytes()?).await?;
        Ok(id)
    }
    async fn id(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=90)
    }
    async fn insert_bytes(&self, key: i32, content: Bytes) -> Result<(), crate::Error> {
        let key: Bytes = Bytes::copy_from_slice(&key.to_le_bytes());
        // let desc: Bytes = Bytes::copy_from_slice(desc.as_bytes());
        self.doc.set_bytes(self.author, key, content).await?;
        Ok(())
    }
}

impl Todo {
    fn as_bytes(&self) -> Result<Bytes, crate::Error> {
        let buf = json::to_vec(self)?;
        Ok(buf.into())
    }
}
