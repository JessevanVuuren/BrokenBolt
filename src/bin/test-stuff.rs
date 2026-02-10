use std::{
    env,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use broken_bolt::Kraken;
use reqwest::{Client, header::HeaderMap};
use serde_json::Value;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let res: Value = fetch_auth(BALANCE_URL, &Value::Null).await?;

    // let res: ServerTime = Fetch2::get(SERVER_TIME_URL)?.send().await?;

    let mut kraken = Kraken::from_env().expect("msg");

    let res = kraken.get_asset_pair("BTC/EUR").await?;

    println!("{:#?}", res);

    Ok(())
}
