use reqwest;
use serde::Deserialize;
use serde_json;
use std::{error, fmt};


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TelegraphError {
    ApiError(String),
    #[serde(skip)]
    RequestError(reqwest::Error),
    #[serde(skip)]
    ParseError(serde_json::Error)
}


impl fmt::Display for TelegraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelegraphError::ApiError(error) => write!(f, "Api error: {}", error),
            TelegraphError::RequestError(error) => write!(f, "Request error: {}", error),
            TelegraphError::ParseError(error) => write!(f, "Parse error: {}", error),
        }
    }
}


impl error::Error for TelegraphError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TelegraphError::RequestError(error) => Some(error),
            TelegraphError::ApiError(_) => None,
            TelegraphError::ParseError(error) => Some(error),
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
