use thiserror::Error;

use crate::CreateSignError;

#[derive(Debug, Error)]
pub enum FetchError {
    #[error(transparent)]
    ParsePayload(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    GetHttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),

    #[error(transparent)]
    Parse(#[from] NestedParseError),
}

#[derive(Debug, Error)]
pub enum AuthFetchError {
    #[error(transparent)]
    ParsePayload(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    GetHttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    ParseJson(#[from] serde_json::Error),

    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    
    #[error(transparent)]
    Parse(#[from] NestedParseError),

    #[error(transparent)]
    Auth(#[from] CreateSignError),
}

#[derive(Debug, Error)]
pub enum KrakenEnvError {
    #[error("Missing environment variable: {0}")]
    EnvError(String),
}

#[derive(Debug, Error)]
pub enum NestedParseError {
    #[error("Missing String field: {0}")]
    MissingField(String),

    #[error(transparent)]
    ParseJSON(#[from] serde_json::Error),
}
