use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    handler::{
        candle::Candle,
        orderbook::{self, OrderBook},
    },
    types::{
        types::{OrderBookData, OrderBookType},
    },
    utils::{decode_fixed, encode_fixed},
};

pub struct App {
    pub orderbook: OrderBook,
    pub candle: Candle,
}

impl App {
    pub fn new(orderbook: OrderBook, candle: Candle) -> App {
        App {
            orderbook,
            candle,
        }
    }
}
