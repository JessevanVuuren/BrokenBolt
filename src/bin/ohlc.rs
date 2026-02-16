use std::error::Error;

use broken_bolt::{Ch, Channel, Socket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ohlc_channel = Channel::new(Ch::OHLC, vec!["BTC/EUR"], None);
    let mut web = Socket::new(vec![ohlc_channel]);
    
    web.start().await.expect("No not good");
    web.subscribe_to_channels(true).await;

    tokio::spawn(async move {
        let mut msg = web.recv_msg.take().expect("incomming error");
        while let Some(data) = msg.recv().await {
            println!("{}", data.message);
        }
    }).await;

    

    Ok(())
}
