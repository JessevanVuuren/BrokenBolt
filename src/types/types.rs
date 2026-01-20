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
    pub result: AckResult,
    pub success: bool,
    #[serde(rename = "time_in")]
    pub time_in: String,
    #[serde(rename = "time_out")]
    pub time_out: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AckResult {
    pub channel: String,
    pub snapshot: bool,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerType {
    pub channel: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Vec<TickerData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    pub symbol: String,
    pub bid: f64,
    #[serde(rename = "bid_qty")]
    pub bid_qty: f64,
    pub ask: f64,
    #[serde(rename = "ask_qty")]
    pub ask_qty: f64,
    pub last: f64,
    pub volume: f64,
    pub vwap: f64,
    pub low: f64,
    pub high: f64,
    pub change: f64,
    #[serde(rename = "change_pct")]
    pub change_pct: f64,
    #[serde(rename = "volume_usd")]
    pub volume_usd: f64,
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookType {
    pub channel: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Vec<OrderBookData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookData {
    pub symbol: String,
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
    pub checksum: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub price: f64,
    pub qty: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    pub price: f64,
    pub qty: f64,
}
