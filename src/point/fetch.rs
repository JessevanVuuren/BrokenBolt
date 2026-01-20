use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("failed to serialize payload")]
    ParsePayload(#[from] serde_urlencoded::ser::Error),

    #[error("failed to decode base64 private key")]
    ParseBase64(#[from] base64::DecodeError),

    #[error("failed to fetch data from request")]
    GetHttpRequest(#[from] reqwest::Error),

    #[error("failed to parse url")]
    ParseUrl(#[from] ParseError),
}

pub async fn fetch_params<T: DeserializeOwned>(url: &str, params: Vec<(&str, &str)>) -> Result<T, FetchError> {
    let url = Url::parse_with_params(url, params)?;

    #[rustfmt::skip]
    let res: T = Client::new().post(url).send().await?.json().await?;

    Ok(res)
}
