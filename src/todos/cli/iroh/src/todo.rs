use std::str::FromStr;

use indicatif::HumanBytes;
use iroh::docs::store::Query;
use iroh::{
    base::base32,
    client::{docs::Entry, Doc, Iroh},
    docs::{AuthorId, NamespaceSecret},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

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
    pub async fn add(&self, description: &str) -> Result<i32, crate::Error> {
        let id = Self::gen_id();
        self.insert(id.to_string(), description.to_owned()).await?;
        Ok(id)
    }
    pub async fn list(&self) -> Result<(), crate::Error> {
        let mut stream = self.document.get_many(Query::all()).await.unwrap();
        while let Some(entry) = stream.try_next().await.unwrap() {
            println!("\nentry {}", Self::fmt_entry(&entry));
            println!("  document id: {}", self.document.id());
            println!("  author id: {}", entry.author());
            let content = entry.content_bytes(&self.document).await.unwrap();
            println!("  content {}", std::str::from_utf8(&content).unwrap())
        }
        Ok(())
    }
    fn fmt_entry(entry: &Entry) -> String {
        let id = entry.id();
        let key = std::str::from_utf8(id.key()).unwrap_or("<bad key>");
        let author = id.author().fmt_short();
        let hash = entry.content_hash();
        println!("hash: {hash}");
        let hash = base32::fmt_short(hash.as_bytes());
        let len = HumanBytes(entry.content_len());
        format!("@{author}: {key} = {hash} ({len})",)
    }
    fn gen_id() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=90)
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
            .import_namespace(iroh::docs::Capability::Write(NamespaceSecret::from_str(
                namespace_id,
            )?))
            .await?;
        Ok(document)
    }
}
