use std::{
    env,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use broken_bolt::{BalanceType, Kraken, TradeHistoryBody, pp_json};
use dotenv::dotenv;
use reqwest::{Client, header::HeaderMap};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let res: Value = fetch_auth(BALANCE_URL, &Value::Null).await?;
    // let res: ServerTime = Fetch2::get(SERVER_TIME_URL)?.send().await?;

    let mut kraken = Kraken::from_env().expect("unable to load env vars, auth wont be available");

    // let res = kraken.get_asset_pair("BTC/EUR").await?;

    // let res = kraken.get_balance_trade(BalanceType::Base, "EUR").await?;

    let res = kraken.get_trades_history(&TradeHistoryBody::default()).await?;

    
    // println!("{:#?}", res);
    // pp_json(&res);

    Ok(())
}
