use std::num::ParseFloatError;

use serde_json::Value;

use crate::{
    KraSoc, Kraken,
    fetch::{error::FetchError, types::AssetPairs},
    pp_json, rfc3339_to_epoch,
    types::types::CandleStick,
    utils::{NestedParseError, epoch_to_rfc3339, nested_object},
};

#[derive(Debug, Clone)]
pub struct Candle {
    pair: String,
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
            pair: pair.into(),
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
                interval_begin: epoch_to_rfc3339(raw_candle.0),
                epoch: raw_candle.0,
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

    pub fn web_stream(&mut self, data: KraSoc<CandleStick>) {
        if data.type_field == "snapshot" {
            self.verify_websocket_snapshot(&data.data);
        }

        if data.type_field == "update" {
            self.append_streaming_data(&data.data[0]);
        }
    }

    fn verify_websocket_snapshot(&self, data: &[CandleStick]) {
        if (data[0].interval != self.interval) {
            panic!("Fetch and Socket interval does not match");
        }

        if (data[0].symbol != self.pair) {
            panic!("Fetch and Socket pair does not match");
        }

        for (fetch, socket) in self.candles.iter().zip(data.iter().rev()) {
            let epoch_fetch = fetch.epoch;
            let epoch_socket = rfc3339_to_epoch(&socket.interval_begin);

            if epoch_fetch != epoch_socket {
                panic!("Fetch and Socket candles are out of sync");
            }
        }
    }

    fn append_streaming_data(&mut self, data: &CandleStick) {
        let epoch_fetch = self.candles[0].epoch;
        let epoch_socket = rfc3339_to_epoch(&data.interval_begin);

        if epoch_fetch == epoch_socket {
            self.candles[0].high = data.high;
            self.candles[0].low = data.low;
            self.candles[0].close = data.close;
            self.candles[0].vwap = data.vwap;
            self.candles[0].volume = data.volume;
            self.candles[0].trades = data.trades;
        } else {
            let mut new_candle = data.clone();
            new_candle.epoch = epoch_socket;
            self.candles.insert(0, new_candle);

            if self.candles.len() > 700 {
                self.candles.split_off(700);
            }
        }
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
