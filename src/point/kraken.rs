use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use thiserror::Error;
use url::{ParseError, Url};

use crate::point::{
    error::{FetchError, KrakenEnvError, NestedParseError},
    types::{AssetPairs, RawCandleStick, ServerTime},
};

pub const BASE_URL: &str = "https://api.kraken.com/0";
pub const SERVER_TIME_URL: &str = "https://api.kraken.com/0/public/Time";
pub const ASSET_PAIRS_URL: &str = "https://api.kraken.com/0/public/AssetPairs";
pub const OHLC_URL: &str = "https://api.kraken.com/0/public/OHLC";

pub struct Kraken {
    public_key: String,
    private_key: String,
    client: Client,
}

#[derive(Serialize)]
struct AuthBody<'a, T> {
    nonce: u128,
    #[serde(flatten)]
    body: &'a T,
}

impl Kraken {
    pub fn from_env() -> Result<Self, KrakenEnvError> {
        dotenv().ok();

        let public_key = std::env::var("PUBLIC_KEY").map_err(|_| KrakenEnvError::EnvError("PUBLIC_KEY".to_string()))?;
        let private_key = std::env::var("PRIVATE_KEY").map_err(|_| KrakenEnvError::EnvError("PRIVATE_KEY".to_string()))?;

        Ok(Self {
            public_key,
            private_key,
            client: Client::new(),
        })
    }

    pub fn new(public_key: &str, private_key: &str) -> Self {
        Self {
            public_key: public_key.into(),
            private_key: private_key.into(),
            client: Client::new(),
        }
    }

    pub async fn get_server_time(&self) -> Result<ServerTime, FetchError> {
        let res: ServerTime = Client::new().post(SERVER_TIME_URL).send().await?.json().await?;
        Ok(res)
    }

    pub async fn get_asset_pair(&self, pair: &str) -> Result<AssetPairs, FetchError> {
        let params = vec![("pair", pair.to_owned())];
        let url = Url::parse_with_params(ASSET_PAIRS_URL, params)?;

        let mut res: Value = self.client.post(url).send().await?.json().await?;

        let path = format!("/result/{}", pair.replace('/', "~1"));
        let assets: AssetPairs = Self::nested(&path, &mut res)?;

        Ok(assets)
    }

    pub async fn get_ohlc(&self, pair: &str, interval: &str, since: &str) -> Result<Vec<RawCandleStick>, FetchError> {
        let params = vec![
            ("pair", pair.to_owned()),
            ("interval", interval.to_string()),
            ("since", since.to_string()),
        ];

        let url = Url::parse_with_params(OHLC_URL, params)?;
        let mut res: Value = self.client.post(url).send().await?.json().await?;
        let path = format!("/result/{}", pair.replace('/', "~1"));
        let raw_sticks: Vec<RawCandleStick> = Self::nested(&path, &mut res)?;

        Ok(raw_sticks)
    }

    pub fn nested<T: DeserializeOwned>(path: &str, json: &mut Value) -> Result<T, NestedParseError> {
        let value = json.pointer_mut(path).ok_or(NestedParseError::MissingField(path.to_string()))?;
        let asset: T = serde_json::from_value(value.take())?;
        Ok(asset)
    }

    fn get_nonce() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Unable to get time").as_millis()
    }
}
