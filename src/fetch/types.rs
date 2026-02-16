use core::fmt;

use nestify::nest;
use serde::de::value::StrDeserializer;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::fetch::utils::str_to_f64;

pub type RawCandleStick = (u64, String, String, String, String, String, String, i64);

// kraken response
#[derive(Debug, Serialize, Deserialize)]
pub struct KraRre<T> {
    pub error: Vec<String>,
    pub result: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPairs {
    pub altname: String,
    pub wsname: String,
    #[serde(rename = "aclass_base")]
    pub aclass_base: String,
    pub base: String,
    #[serde(rename = "aclass_quote")]
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    #[serde(rename = "cost_decimals")]
    pub cost_decimals: i32,
    #[serde(rename = "pair_decimals")]
    pub pair_decimals: i32,
    #[serde(rename = "lot_decimals")]
    pub lot_decimals: i32,
    #[serde(rename = "lot_multiplier")]
    pub lot_multiplier: i32,
    #[serde(rename = "leverage_buy")]
    pub leverage_buy: Vec<i64>,
    #[serde(rename = "leverage_sell")]
    pub leverage_sell: Vec<i64>,
    pub fees: Vec<Vec<f64>>,
    #[serde(rename = "fees_maker")]
    pub fees_maker: Vec<Vec<f64>>,
    #[serde(rename = "fee_volume_currency")]
    pub fee_volume_currency: String,
    #[serde(rename = "margin_call")]
    pub margin_call: i64,
    #[serde(rename = "margin_stop")]
    pub margin_stop: i64,
    pub ordermin: String,
    pub costmin: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    pub status: String,
    #[serde(rename = "long_position_limit")]
    pub long_position_limit: i64,
    #[serde(rename = "short_position_limit")]
    pub short_position_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub unixtime: i64,
    pub rfc1123: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(deserialize_with = "str_to_f64")]
    pub balance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceEx {
    #[serde(deserialize_with = "str_to_f64")]
    pub balance: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub credit: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub credit_used: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub hold_trade: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceTrade {
    #[serde(deserialize_with = "str_to_f64")]
    pub eb: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub tb: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub m: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub uv: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub n: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub c: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub v: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub e: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub mf: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub mfo: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: String,
    #[serde(rename = "ordertxid")]
    pub order_txid: String,
    #[serde(rename = "postxid")]
    pub post_xid: String,
    pub pair: String,
    pub aclass: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ordertype: String,
    #[serde(rename = "tradeordertype")]
    pub trade_ordertype: String,
    #[serde(deserialize_with = "str_to_f64")]
    pub price: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub cost: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub fee: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub vol: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub margin: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub leverage: f64,
    pub misc: String,
    #[serde(rename = "trade_id")]
    pub trade_id: i64,
    pub maker: bool,
}