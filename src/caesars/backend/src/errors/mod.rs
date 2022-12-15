pub mod app;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<axum::Error> for Error {
    fn from(err: axum::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Error::NotFound("env var not found".into()),
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<async_graphql::Error> for Error {
    fn from(err: async_graphql::Error) -> Self {
        Error::Internal(err.message)
    }
}

impl std::convert::From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
