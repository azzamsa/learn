use std::path::Path;
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub document_id: String,
    pub author_id: String,
}

impl Config {
    pub fn new(document_id: &str, author_id: &str) -> Self {
        Self {
            document_id: document_id.to_string(),
            author_id: author_id.to_string(),
        }
    }
    /// Return a configuration struct
    pub fn read() -> Result<Self, Error> {
        let file_content = fs::read_to_string(Self::path())?;
        Self::deserialize(&file_content)
    }
    pub fn write(&self) -> Result<(), crate::Error> {
        let config = toml::to_string(&self)?;
        fs::write(Self::path(), config)?;
        Ok(())
    }
    /// Convert config string into a struct
    fn deserialize(content: &str) -> Result<Self, crate::Error> {
        match toml::from_str(content) {
            Ok(config) => Ok(config),
            Err(e) => Err(crate::Error::Internal(e.to_string())),
        }
    }
    pub fn storage_path() -> PathBuf {
        Path::new(env!("HOME"))
            .join(".local")
            .join("share")
            .join("todos-iroh")
    }
    fn path() -> PathBuf {
        Self::storage_path().join("config.toml")
    }
}
