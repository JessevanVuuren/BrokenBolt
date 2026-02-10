use nestify::nest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

nest! {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]*
    #[serde(rename_all = "camelCase")]*
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
}

nest! {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]*
    #[serde(rename_all = "camelCase")]*
    pub struct ServerTime {
        pub error: Vec<String>,
        pub result: pub struct Result {
            pub unixtime: i64,
            pub rfc1123: String,
        },
    }
}

pub type RawCandleStick = (u64, String, String, String, String, String, String, i64);
