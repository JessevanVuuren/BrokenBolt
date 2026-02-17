use std::num::ParseFloatError;

use serde_json::Value;

use crate::{
    Kraken, fetch::{error::FetchError, types::AssetPairs}, pp_json, types::{candle::CandleStick, types::OhlcData}, utils::{NestedParseError, epoch_to_string, nested_object}
};

#[derive(Debug, Clone)]
pub struct Candle {
    interval: i64,
    asset_pair: AssetPairs,
    pub candles: Vec<CandleStick>,
}

type RawCandleStick = (u64, String, String, String, String, String, String, i64);

#[derive(Debug, thiserror::Error)]
pub enum InitCandleError {
    #[error(transparent)]
    FetchCandle(#[from] FetchError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
}

impl Candle {
    pub async fn new(kraken: &Kraken, pair: &str, interval: i64) -> Result<Self, InitCandleError> {
        let asset_pair = kraken.get_asset_pair(pair).await?;
        let raw_sticks = kraken.get_ohlc(pair, &interval.to_string(), "0").await?;
        let candles = Self::build_candle_sticks(raw_sticks, pair, interval)?;

        Ok(Self {
            interval,
            asset_pair,
            candles,
        })
    }

    fn build_candle_sticks(raw_candles: Vec<RawCandleStick>, pair: &str, interval: i64) -> Result<Vec<CandleStick>, ParseFloatError> {
        let mut candles: Vec<CandleStick> = Vec::new();

        for raw_candle in raw_candles {
            candles.push(CandleStick {
                symbol: pair.to_string(),
                open: raw_candle.1.parse()?,
                high: raw_candle.2.parse()?,
                low: raw_candle.3.parse()?,
                close: raw_candle.4.parse()?,
                trades: raw_candle.7,
                volume: raw_candle.6.parse()?,
                vwap: raw_candle.5.parse()?,
                interval_begin: epoch_to_string(raw_candle.0),
                interval: interval,
                timestamp: String::new(),
            });
        }

        candles.reverse();

        Ok(candles)
    }

    pub fn ohlc(&self, index: usize) -> [f64; 4] {
        [
            self.candles[index].open,
            self.candles[index].high,
            self.candles[index].low,
            self.candles[index].close,
        ]
    }

    pub fn web_stream(&self, data: Vec<OhlcData>) {
        if (data[0].interval != self.interval) {
            panic!("Interval does not match fetch: {}, socket: {}", self.interval, data[0].interval)
        }

        if (data.len() > 1) {
            panic!("To many updates in OHLC candles")
        }

    
        println!("begin fetch : {}", self.candles[0].interval_begin);
        println!("begin socket: {}", data[0].interval_begin);
    }

    pub fn min_max(&self, mut depth: usize) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        if depth == 0 {
            depth = self.candles.len();
        }
        for c in self.candles.iter().take(depth) {
            min = min.min(c.open).min(c.high).min(c.low).min(c.close);
            max = max.max(c.open).max(c.high).max(c.low).max(c.close);
        }

        (min, max)
    }

    pub fn print_ohlc(&self, depth: usize) {
        for c in self.candles.iter().take(depth) {
            println!("{}, {}, {}, {}", c.open, c.high, c.low, c.close)
        }
    }

    pub fn print_candles(&self, depth: usize) {
        for candle in self.candles.iter().take(depth) {
            println!(
                "{}, open: {}, high: {}, low: {}, close: {}, volume: {}",
                &candle.interval_begin[0..19],
                candle.open,
                candle.high,
                candle.low,
                candle.close,
                candle.volume,
            )
        }
    }
}
