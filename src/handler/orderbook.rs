use std::{cmp::Reverse, collections::BTreeMap, fmt::format, ops::Index, process::exit};

use serde_json::Value;

use crate::{
    point::fetch::{FetchError, fetch_params},
    types::{
        points::AssetPairs,
        types::{OrderBookData, OrderBookType},
    },
    urls::ASSET_PAIRS_URL,
    utils::{NestedParseError, decode_fixed, encode_fixed, nested_object},
};

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub asset_info: AssetPairs,
    pub asks: BTreeMap<i64, i64>,
    pub bids: BTreeMap<Reverse<i64>, i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum OrderBookError {
    #[error(transparent)]
    Fetch(#[from] FetchError),

    #[error(transparent)]
    Parse(#[from] NestedParseError),
}

impl OrderBook {
    pub async fn new(pair: &str) -> Result<Self, OrderBookError> {
        let params = vec![("pair", pair)];
        let mut data: Value = fetch_params(ASSET_PAIRS_URL, params).await?;
        let path = format!("/result/{}", pair.replace('/', "~1"));
        let assets: AssetPairs = nested_object(&path, &mut data)?;

        Ok(Self {
            asset_info: assets,
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
        let price_precision = self.asset_info.pair_decimals;
        let qty_precision = self.asset_info.lot_decimals;

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
        let price_precision = self.asset_info.pair_decimals;
        let qty_precision = self.asset_info.lot_decimals;

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

    pub fn print_table(&mut self, size: u8) {
        let price_precision = self.asset_info.pair_decimals;
        let qty_precision = self.asset_info.lot_decimals;
        println!("Update orderbook, bid: {}, ask: {}", self.bids.len(), self.asks.len());

        for entry in self.bids.iter() {
            let price = decode_fixed(price_precision, entry.0.0);
            let qty = decode_fixed(qty_precision, entry.1.clone());

            println!("Price: {}, Bid: {}", price, qty);
        }

        println!();

        for entry in self.asks.iter() {
            let price = decode_fixed(price_precision, entry.0.clone());
            let qty = decode_fixed(qty_precision, entry.1.clone());

            println!("Price: {}, Bid: {}", price, qty);
        }
        println!();
        println!();
    }
}
