use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json_fmt::JsonSyntaxError;
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
