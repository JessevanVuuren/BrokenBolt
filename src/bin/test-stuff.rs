use std::{
    env,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use broken_bolt::{BALANCE_URL, Nonce, ServerTime, fetch_auth, fetch_params, get_kraken_signature};
use reqwest::{Client, header::HeaderMap};
use serde_json::Value;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let res: Value = fetch_auth(BALANCE_URL, &Value::Null).await?;

    // let private_key = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY key found");
    // let public_key = env::var("PUBLIC_KEY").expect("No PUBLIC_KEY key found");

    // let key = get_kraken_signature("/0/private/Balance", &body, &private_key, nonce)?;

    // let mut headers: HeaderMap = HeaderMap::new();
    // headers.insert("API-Key", public_key.parse().unwrap());
    // headers.insert("API-Sign", key.parse().unwrap());

    // let res: Value = Client::new().post(BALANCE_URL).headers(headers).json(&body).send().await?.json().await?;

    println!("{:#?}", res);

    Ok(())
}
