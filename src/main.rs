use std::process::exit;
use std::thread;
use std::time::Duration;

use tokio_tungstenite::tungstenite::Message;

use crate::handler::orderbook::{self, OrderBook};
use crate::point::fetch::fetch_params;
use crate::socket::socket::Incoming;
use crate::socket::{channels::Channel, socket::Socket};
use crate::types::points::AssetPairs;
use crate::types::types::{OrderBookType, TickerType};
use crate::urls::WEBSOCKET_URL;

mod handler;
mod point;
mod socket;
mod types;
mod urls;
mod utils;

#[tokio::main]
async fn main() {
    // let ticker_channel = Channel::new("ticker", vec!["BTC/USD"]);
    let orderbook_channel = Channel::new("book", vec!["BTC/EUR"]);

    let mut web = Socket::new(vec![orderbook_channel]);

    web.start(WEBSOCKET_URL).await.expect("Error socket {}");
    web.subscribe_to_channels().await;

    let mut orderbook = OrderBook::new("BTC/EUR").await.expect("Failed to init orderbook");

    let main = tokio::spawn(async move {
        let mut msg = web.recv_msg.take().expect("msg");
        while let Some(data) = msg.recv().await {
            if (data.channel == "subscribe" || data.channel == "heartbeat" || data.channel == "status") {
                continue;
            }
            incoming(data, &mut web, &mut orderbook).await
        }
    });

    main.await;
}

async fn incoming(msg: Incoming, soc: &mut Socket, orderbook: &mut OrderBook) {
    // println!("Channel: {}, pair: {}", msg.channel, msg.pair);

    if msg.channel == "ticker" {
        let ticker: TickerType = serde_json::from_str(&msg.message.to_string()).unwrap();
        // println!("{:?}", ticker);
    }

    if msg.channel == "book" {
        let ob_data: OrderBookType = serde_json::from_str(&msg.message.to_string()).unwrap();
        orderbook.stream(ob_data);
        orderbook.print_table(10);
    }
}
