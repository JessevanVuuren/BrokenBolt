use std::{
    collections::HashMap,
    env,
    os::raw,
    time::{SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use reqwest::{Client, header::HeaderMap};
use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, Error},
};
use serde_json::{Value, json};
use thiserror::Error;
use url::{ParseError, Url};

use crate::{
    BalanceType, CreateSignError,
    fetch::{
        body::TradeHistoryBody,
        error::{AuthFetchError, FetchError, KrakenEnvError, NestedParseError},
        types::{AssetPairs, Balance, BalanceEx, BalanceTrade, KraRre, RawCandleStick, ServerTime, Trade},
        urls::{ASSET_PAIRS_URL, BALANCE_EX_URL, BALANCE_TRADE_URL, BALANCE_URL, BASE_URL, OHLC_URL, SERVER_TIME_URL, TRADES_HISTORY_URL},
    },
    get_kraken_signature, pp_json,
};

pub struct Kraken {
    public_key: String,
    private_key: String,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthBody<T> {
    nonce: u128,
    #[serde(flatten)]
    body: T,
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

    pub fn nested<T: DeserializeOwned>(path: &str, json: &mut Value) -> Result<T, NestedParseError> {
        let value = json.pointer_mut(path).ok_or(NestedParseError::MissingField(path.to_string()))?;
        let asset: T = serde_json::from_value(value.take())?;
        Ok(asset)
    }

    fn get_nonce() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Unable to get time").as_millis()
    }

    fn build_url(end_point: &str) -> Result<Url, ParseError> {
        let url = format!("{}{}", BASE_URL, end_point);
        return Url::parse(&url);
    }

    fn build_params_url(end_point: &str, params: Vec<(&str, &str)>) -> Result<Url, ParseError> {
        let url = format!("{}{}", BASE_URL, end_point);
        return Url::parse_with_params(&url, params);
    }

    fn auth_headers<T: Serialize>(&self, url: &str, body: &AuthBody<T>) -> Result<HeaderMap, CreateSignError> {
        let sign = get_kraken_signature(url, body, &self.private_key, body.nonce)?;

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("API-Key", self.public_key.parse().unwrap());
        headers.insert("API-Sign", sign.parse().unwrap());

        Ok(headers)
    }

    fn body_to_auth<T: Serialize>(body: T) -> AuthBody<T> {
        AuthBody {
            nonce: Self::get_nonce(),
            body,
        }
    }

    pub async fn get_server_time(&self) -> Result<ServerTime, FetchError> {
        let res: ServerTime = Client::new().post(SERVER_TIME_URL).send().await?.json().await?;
        Ok(res)
    }

    pub async fn get_asset_pair(&self, pair: &str) -> Result<AssetPairs, FetchError> {
        let url = Self::build_params_url(ASSET_PAIRS_URL, vec![("pair", pair)])?;
        let mut res: KraRre<Value> = self.client.post(url).send().await?.json().await?;

        let path = format!("/{}", pair.replace('/', "~1"));
        let assets: AssetPairs = Self::nested(&path, &mut res.result)?;

        Ok(assets)
    }

    pub async fn get_ohlc(&self, pair: &str, interval: &str, since: &str) -> Result<Vec<RawCandleStick>, FetchError> {
        let params = vec![("pair", pair), ("interval", interval), ("since", since)];
        let url = Self::build_params_url(OHLC_URL, params)?;

        let mut res: KraRre<Value> = self.client.post(url).send().await?.json().await?;

        let path = format!("/{}", pair.replace('/', "~1"));
        let raw_sticks: Vec<RawCandleStick> = Self::nested(&path, &mut res.result)?;

        Ok(raw_sticks)
    }

    pub async fn get_balance(&self, multiplier: BalanceType) -> Result<KraRre<Balance>, AuthFetchError> {
        let extra = ("rebase_multiplier", multiplier.to_string());
        let body = Self::body_to_auth(HashMap::from([extra]));

        let url = Self::build_url(BALANCE_URL)?;
        let headers = self.auth_headers(BALANCE_URL, &body)?;

        let mut res: KraRre<Value> = Client::new().post(url).headers(headers).json(&body).send().await?.json().await?;
        let assets: f64 = Self::nested(&"/ZEUR", &mut res.result)?;

        Ok(KraRre {
            error: res.error,
            result: Balance {
                balance: assets,
            },
        })
    }

    pub async fn get_balance_ex(&self, multiplier: BalanceType) -> Result<KraRre<BalanceEx>, AuthFetchError> {
        let extra = ("rebase_multiplier", multiplier.to_string());
        let body = Self::body_to_auth(HashMap::from([extra]));

        let url = Self::build_url(BALANCE_EX_URL)?;
        let headers = self.auth_headers(BALANCE_EX_URL, &body)?;

        let mut res: KraRre<Value> = Client::new().post(url).headers(headers).json(&body).send().await?.json().await?;

        let assets: BalanceEx = Self::nested(&"/ZEUR", &mut res.result)?;

        Ok(KraRre {
            error: res.error,
            result: assets,
        })
    }

    pub async fn get_balance_trade(&self, multiplier: BalanceType, asset: &str) -> Result<KraRre<BalanceTrade>, AuthFetchError> {
        let extra = [("rebase_multiplier", multiplier.to_string()), ("asset", asset.to_string())];
        let body = Self::body_to_auth(HashMap::from(extra));

        let url = Self::build_url(BALANCE_TRADE_URL)?;
        let headers = self.auth_headers(BALANCE_TRADE_URL, &body)?;

        let mut res: KraRre<BalanceTrade> = Client::new().post(url).headers(headers).json(&body).send().await?.json().await?;

        Ok(res)
    }

    pub async fn get_trades_history(&self, params: &TradeHistoryBody) -> Result<Vec<Trade>, AuthFetchError> {
        let body = Self::body_to_auth(params);

        let url = Self::build_url(TRADES_HISTORY_URL)?;
        let headers = self.auth_headers(TRADES_HISTORY_URL, &body)?;

        let mut res: KraRre<Value> = Client::new().post(url).headers(headers).json(&body).send().await?.json().await?;
        let mut raw_trades: Value = Self::nested(&"/trades", &mut res.result)?;

        let mut obj_trades = raw_trades.as_object_mut().ok_or(serde_json::Error::custom("Unable to parse trades"))?;

        let trades = obj_trades
            .iter_mut()
            .map(|(key, value)| {
                value["id"] = json!(key);
                serde_json::from_value(value.take())
            })
            .collect::<Result<Vec<Trade>, _>>()?;

        Ok(trades)
    }
}
