use serde_json::Value;

use crate::{
    error::error::AssetPairError,
    point::{asset_pair::get_asset_pair, fetch::fetch_params},
    types::{candle::CandleStick, points::AssetPairs},
    urls::ASSET_PAIRS_URL,
    utils::nested_object,
};

pub struct Candle {
    pub asset_pair: AssetPairs,
    candles: Vec<CandleStick>,
}

impl Candle {
    pub async fn new(pair: &str, interval: i32, since: i32) -> Result<Self, AssetPairError> {
        let asset_pair = get_asset_pair(pair).await?;
        Ok(Self {
            asset_pair,
            candles: vec![],
        })
    }
}
