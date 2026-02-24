use std::{cmp::Reverse, collections::BTreeMap, fmt::format, ops::Index, process::exit, sync::Arc};

use serde_json::Value;

use crate::{
    Kraken,
    fetch::{error::FetchError, types::AssetPairs},
    types::types::{OrderBookData, OrderBookType},
    utils::{NestedParseError, decode_fixed, encode_fixed, nested_object},
};

#[derive(Debug, Clone)]
pub struct OrderBook {
    kraken: Arc<Kraken>,
    pub asset_pair: AssetPairs,
    pub asks: BTreeMap<i64, i64>,
    pub bids: BTreeMap<Reverse<i64>, i64>,
}

impl OrderBook {
    pub async fn new(kraken: Arc<Kraken>, pair: &str) -> Result<Self, FetchError> {
        let asset_pair = kraken.get_asset_pair(pair).await?;

        Ok(Self {
            kraken,
            asset_pair,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        })
    }

    pub fn stream(&mut self, data: OrderBookType) {
        self.update(&data.data[0]);

        if let Some((&key, _)) = self.asks.iter().nth(10) {
            self.asks.split_off(&key);
        }

        if let Some((&key, _)) = self.bids.iter().nth(10) {
            self.bids.split_off(&key);
        }

        if (self.checksum() != data.data[0].checksum) {
            panic!("ERROR: checksum did not match")
        }
    }

    fn update(&mut self, data: &OrderBookData) {
        let price_precision = self.asset_pair.pair_decimals;
        let qty_precision = self.asset_pair.lot_decimals;

        for bid in data.bids.iter() {
            let key = encode_fixed(price_precision, bid.price);
            let qty = encode_fixed(qty_precision, bid.qty);

            match qty {
                0 => self.bids.remove(&Reverse(key)),
                _ => self.bids.insert(Reverse(key), qty),
            };
        }

        for ask in data.asks.iter() {
            let key = encode_fixed(price_precision, ask.price);
            let qty = encode_fixed(qty_precision, ask.qty);

            match qty {
                0 => self.asks.remove(&key),
                _ => self.asks.insert(key, qty),
            };
        }
    }

    pub fn checksum(&mut self) -> u32 {
        let price_precision = self.asset_pair.pair_decimals;
        let qty_precision = self.asset_pair.lot_decimals;

        let mut ask_string = String::new();
        for entry in self.asks.iter() {
            let str = format!("{}{}", entry.0, entry.1.clone());
            ask_string.push_str(&str);
        }

        let mut bid_string = String::new();
        for entry in self.bids.iter() {
            let str = format!("{}{}", entry.0.0, entry.1.clone());
            bid_string.push_str(&str);
        }

        let final_string = format!("{}{}", ask_string, bid_string);
        crc32fast::hash(final_string.as_bytes())
    }

    pub fn price_decoded(&self, price: i64) -> f64 {
        decode_fixed(self.asset_pair.pair_decimals, price)
    }

    pub fn qty_decoded(&self, qty: i64) -> f64 {
        decode_fixed(self.asset_pair.lot_decimals, qty)
    }

    pub fn ask_decode(&self, pair: (&i64, &i64)) -> (f64, f64) {
        (self.price_decoded(*pair.0), self.qty_decoded(*pair.1))
    }

    pub fn bid_decode(&self, pair: (&Reverse<i64>, &i64)) -> (f64, f64) {
        (self.price_decoded(pair.0.0), self.qty_decoded(*pair.1))
    }

    pub fn print_table(&mut self, size: u8) {
        println!("Update orderbook, bid: {}, ask: {}", self.bids.len(), self.asks.len());

        for entry in self.bids.iter() {
            let (price, qty) = self.bid_decode(entry);
            println!("Price: {}, Bid: {}", price, qty);
        }

        println!();

        for entry in self.asks.iter() {
            let (price, qty) = self.ask_decode(entry);
            println!("Price: {}, Bid: {}", price, qty);
        }
        println!();
        println!();
    }
}
