use std::{cmp::Reverse, collections::BTreeMap};

use crate::{
    handler::orderbook::{self, OrderBook},
    types::{points::AssetPairs, types::{OrderBookData, OrderBookType}},
    utils::{decode_fixed, encode_fixed},
};

pub struct App {
    pub orderbook: OrderBook,
}

impl App {
    pub fn new(orderbook: OrderBook) -> App {
        App { orderbook }
    }

    pub fn stream(&mut self, data: OrderBookType) {
        self.orderbook.stream(data);
    }
}
