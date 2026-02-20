use std::{
    env,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use broken_bolt::{AddOrder, BalanceType, Kraken, OrderType, Side, TradeHistoryBody, pp_json};
use dotenv::dotenv;
use reqwest::{Client, header::HeaderMap};
use serde_json::{Value, json};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4().to_string();

    let mut kraken = Kraken::from_env().expect("unable to load env vars, auth wont be available");

    let body = TradeHistoryBody {
        ..Default::default()   
    };
    let res = kraken.get_trades_history(&body).await?;

    pp_json(&res);


    // let order = AddOrder {
    //     cl_ord_id: Some(id),
    //     ordertype: OrderType::Market,
    //     type_field: Side::SELL,
    //     volume: "6".into(),
    //     pair: "XRPEUR".into(),
    //     ..Default::default()
    // };

    // let res = kraken.post_add_order(&order).await?;

    // pp_json(&order);
    // pp_json(&res);

    Ok(())
}
