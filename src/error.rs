//! Errors that occurred while working with the library

use reqwest;
use serde::Deserialize;
use serde_json;
use std::{error, fmt, io};


/// Enum of lib errors
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TelegraphError {
    /// Telegrapth API error
    ApiError(String),
    /// Error occurred when sending the request
    #[serde(skip)]
    RequestError(reqwest::Error),
    /// Error occurred when parsing data
    #[serde(skip)]
    ParseError(serde_json::Error),
    /// Error occurred when working with files
    #[serde(skip)]
    IoError(io::Error)
}


impl fmt::Display for TelegraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelegraphError::ApiError(error) => write!(f, "Api error: {}", error),
            TelegraphError::RequestError(error) => write!(f, "Request error: {}", error),
            TelegraphError::ParseError(error) => write!(f, "Parse error: {}", error),
            TelegraphError::IoError(error) => write!(f, "IO error: {}", error),
        }
    }
}


impl error::Error for TelegraphError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TelegraphError::RequestError(error) => Some(error),
            TelegraphError::ApiError(_) => None,
            TelegraphError::ParseError(error) => Some(error),
            TelegraphError::IoError(error) => Some(error),
        }
    }
}


impl From<serde_json::Error> for TelegraphError {
    fn from(error: serde_json::Error) -> Self {
        TelegraphError::ParseError(error)
    }
}

impl From<reqwest::Error> for TelegraphError {
    fn from(error: reqwest::Error) -> Self {
        TelegraphError::RequestError(error)
    }
}

impl From<io::Error> for TelegraphError {
    fn from(error: io::Error) -> Self {
        TelegraphError::IoError(error)
    }
}
