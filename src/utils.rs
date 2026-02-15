use chrono::Utc;
use chrono::prelude::DateTime;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json_fmt::JsonSyntaxError;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NestedParseError {
    #[error("failed to serialize payload")]
    MissingField(String),

    #[error("failed to fetch data from request")]
    ParseJSON(#[from] serde_json::Error),
}

pub fn nested_object<T: DeserializeOwned>(path: &str, json: &mut Value) -> Result<T, NestedParseError> {
    let value = json.pointer_mut(path).ok_or(NestedParseError::MissingField(path.to_string()))?;
    let asset: T = serde_json::from_value(value.take())?;
    Ok(asset)
}

pub fn encode_fixed(precision: i32, value: f64) -> i64 {
    let scale = 10_f64.powi(precision);
    (value * scale).round() as i64
}

pub fn decode_fixed(precision: i32, value: i64) -> f64 {
    let scale = 10_f64.powi(precision);
    value as f64 / scale
}

pub fn epoch_to_string(time: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(time);

    let datetime = DateTime::<Utc>::from(d);

    datetime.to_rfc3339()
}

pub fn pp_json<T: Serialize>(body: &T) {
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
}
