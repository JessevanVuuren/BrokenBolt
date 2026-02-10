use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum FetchError {
    #[error(transparent)]
    ParsePayload(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    GetHttpRequest(#[from] reqwest::Error),

    #[error(transparent)]
    ParseUrl(#[from] ParseError),

    #[error(transparent)]
    Parse(#[from] NestedParseError),
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
