use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Nonce {
    pub nonce: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketKey {
    pub error: Vec<String>,
    pub result: _Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct _Result {
    pub expires: i64,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ping {
    pub method: String,
    #[serde(rename = "req_id")]
    pub req_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubRequest {
    pub method: String,
    pub params: Params,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub channel: String,
    pub symbol: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeAck {
    pub method: String,
    pub result: _AckResult,
    pub success: bool,
    #[serde(rename = "time_in")]
    pub time_in: String,
    #[serde(rename = "time_out")]
    pub time_out: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct _AckResult {
    pub channel: String,
    pub snapshot: bool,
    pub symbol: String,
}
