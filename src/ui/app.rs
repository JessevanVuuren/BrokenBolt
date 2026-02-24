use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    Trades,
    handler::{
        candle::Candle,
        orderbook::{self, OrderBook},
        trades,
    },
    types::types::{OrderBookData, OrderBookType},
    utils::{decode_fixed, encode_fixed},
};

pub struct App {
    pub orderbook: OrderBook,
    pub candle: Candle,
    pub trades: Trades,
}

pub enum Message {
    UpdateCandlesInterval(i64),
    UpdateCandlesPair(String),
}

impl App {
    pub fn new(orderbook: OrderBook, candle: Candle, trades: Trades) -> App {
        App {
            orderbook,
            candle,
            trades,
        }
    }
}
