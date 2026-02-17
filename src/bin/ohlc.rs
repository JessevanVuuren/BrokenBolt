use std::{collections::HashMap, error::Error};

use broken_bolt::{Candle, Ch, Channel, KraSoc, Kraken, OhlcData, Socket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pair = "BTC/EUR";
    let interval = 5;

    let extra = ("interval".to_string(), serde_json::to_value(interval).unwrap());
    let ohlc_channel = Channel::new(Ch::OHLC, vec![pair], Some(HashMap::from([extra])));
    let mut web = Socket::new(vec![ohlc_channel]);

    let mut kraken = Kraken::from_env()?;
    let candles = Candle::new(&kraken, pair, interval).await.expect("Failed to init candle");

    web.start().await.expect("No not good");
    web.subscribe_to_channels(true).await;

    tokio::spawn(async move {
        let mut msg = web.recv_msg.take().expect("incoming error");
        while let Some(data) = msg.recv().await {
            println!("{}", data.message);
            if (data.channel == "subscribe" || data.channel == "heartbeat" || data.channel == "status") {
                continue;
            }
            if data.channel == "ohlc" {
                let ohlc_data: KraSoc<OhlcData> = serde_json::from_str(&data.message.to_string()).unwrap();
                if (ohlc_data.type_field != "snapshot") {
                    candles.web_stream(ohlc_data.data);
                }
            }
        }
    })
    .await;

    Ok(())
}
