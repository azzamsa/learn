#![deny(unsafe_code)]

pub mod cli;
pub mod config;
pub mod error;
pub mod exit_codes;
pub mod output;

pub mod db;
pub mod todo;

// Aliases
pub use error::Error;
