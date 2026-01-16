use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::Client;
use ring::hmac;

use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::SystemTimeError;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::types::types::{Nonce, WebSocketKey};

const WEBSOCKET_TOKEN: &'static str = "/0/private/GetWebSocketsToken";
const BASE_URL: &'static str = "https://api.kraken.com";

#[derive(Debug, Error)]
pub enum CreateSignError {
    #[error("failed to serialize payload")]
    ParsePayload(#[from] serde_urlencoded::ser::Error),

    #[error("failed to decode base64 private key")]
    ParseBase64(#[from] base64::DecodeError),
}

#[derive(Debug, Error)]
pub enum GetWebSocketError {
    #[error("failed to serialize payload")]
    ParsePayload(#[from] CreateSignError),

    #[error("failed to fetch data from request")]
    GetHttpRequest(#[from] reqwest::Error),

    #[error("SystemTime before UNIX EPOCH!")]
    TimeError(#[from] SystemTimeError),
}

pub fn get_kraken_signature<T: serde::Serialize>(path: &str, payload: &T, private_key: &str, nonce: u128) -> Result<String, CreateSignError> {
    let postdata = format!("{}{}", nonce, json!(payload));
    let hashed_data = Sha256::digest(&postdata);

    let mut message = path.to_string().into_bytes();
    message.extend(hashed_data);

    let base_key = BASE64_STANDARD.decode(&private_key.as_bytes())?;
    let hmac_key = hmac::Key::new(hmac::HMAC_SHA512, &base_key);

    let signed_digest = hmac::sign(&hmac_key, &message);
    let sign_key = BASE64_STANDARD.encode(signed_digest);

    Ok(sign_key)
}

pub async fn get_websocket_token(client: &Client, public_key: &str, private_key: &str) -> Result<WebSocketKey, GetWebSocketError> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    #[rustfmt::skip]
    let body = Nonce { nonce: nonce.to_string() };

    let signature = get_kraken_signature(WEBSOCKET_TOKEN, &body, &private_key, nonce)?;

    #[rustfmt::skip]
    let res: WebSocketKey = client.post(format!("{}{}", BASE_URL, WEBSOCKET_TOKEN))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("API-Key", public_key)
        .header("API-Sign", &signature)
        .json(&body).send().await?.json().await?;

    Ok(res)
}
