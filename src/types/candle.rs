use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleStick {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub trades: i64,
    pub volume: f64,
    pub vwap: f64,
    #[serde(rename = "interval_begin")]
    pub interval_begin: String,
    pub interval: i64,
    pub timestamp: String,
}
