use std::thread;
use std::time::Duration;

use tokio_tungstenite::tungstenite::Message;

use crate::api::{channels::Channel, socket::Socket};

mod api;
mod types;

const WEBSOCKET_URL: &'static str = "wss://ws.kraken.com/v2";

#[tokio::main]
async fn main() {
    let ticker = Channel::new("ticker", vec!["BTC/USD", "BTC/EUR"]);
    let orderbook = Channel::new("book", vec!["BTC/USD", "BTC/EUR"]);

    let mut web = Socket::new(vec![ticker, orderbook]);

    web.start(WEBSOCKET_URL).await.expect("Error socket {}");
    web.subscribe_to_channels().await;

    let main = tokio::spawn(async move {
        let mut msg = web.recv_msg.take().expect("msg");
        while let Some(data) = msg.recv().await {
            incoming(data, &mut web).await
        }
    });

    main.await;
}

async fn incoming(msg: Message, soc: &mut Socket) {
    println!("Data: {}", msg);
}
