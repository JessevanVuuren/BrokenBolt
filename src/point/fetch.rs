use std::{
    env,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use reqwest::{Client, Url, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
use url::ParseError;

use crate::{CreateSignError, get_kraken_signature};

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

#[derive(Debug, Error)]
pub enum FetchAuthError {
    #[error("failed to serialize payload")]
    ParsePayload(#[from] CreateSignError),

    #[error("SystemTime before UNIX EPOCH!")]
    TimeError(#[from] SystemTimeError),

    #[error("failed to fetch data from request")]
    GetHttpRequest(#[from] reqwest::Error),
}

pub async fn fetch_params<T: DeserializeOwned>(url: &str, params: Vec<(&str, String)>) -> Result<T, FetchError> {
    let url = Url::parse_with_params(url, params)?;

    let res: T = Client::new().post(url).send().await?.json().await?;

    Ok(res)
}

#[derive(Serialize)]
struct AuthBody<'a, T> {
    nonce: u128,
    #[serde(flatten)]
    body: &'a T,
}

pub async fn fetch_auth<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<R, FetchAuthError> {
    let private_key = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY found in .env, did you add 'dotenv().ok();'?");
    let public_key = env::var("PUBLIC_KEY").expect("No PRIVATE_KEY found in .env, did you add 'dotenv().ok();'?");

    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let body = AuthBody {
        nonce: nonce,
        body,
    };

    let key = get_kraken_signature("/0/private/Balance", &body, &private_key, nonce)?;

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("API-Key", public_key.parse().unwrap());
    headers.insert("API-Sign", key.parse().unwrap());

    let res: R = Client::new().post(url).headers(headers).json(&body).send().await?.json().await?;

    Ok(res)
}
