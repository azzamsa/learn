#![deny(unsafe_code)]

pub mod cli;
pub mod config;
pub mod error;
pub mod exit_codes;
pub mod output;

// Aliases
pub use error::Error;
