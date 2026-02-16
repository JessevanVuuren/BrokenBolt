use core::fmt;
use std::{collections::HashMap, ffi::os_str::Display, fmt::write};

use serde_json::Value;

use crate::types::types::{Params, SubRequest};

pub enum Ch {
    OHLC,
    TICKER,
    BOOK,
}

impl fmt::Display for Ch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ch::OHLC => write!(f, "ohlc"),
            Ch::TICKER => write!(f, "ticker"),
            Ch::BOOK => write!(f, "book"),
        }
    }
}

pub struct Channel {
    pairs: Vec<String>,
    subscribe: SubRequest,
}

impl Channel {
    pub fn new(channel: Ch, pairs: Vec<&str>, extra: Option<HashMap<String, Value>>) -> Self {
        let pairs: Vec<String> = pairs.iter().map(|p| p.to_string()).collect();
        let subscribe = SubRequest {
            method: "subscribe".into(),
            params: Params {
                channel: channel.to_string(),
                symbol: pairs.clone(),
                extra: extra.unwrap_or(HashMap::new()),
            },
        };

        Self {
            pairs,
            subscribe,
        }
    }

    pub fn subscription(&self) -> String {
        serde_json::to_string(&self.subscribe).unwrap()
    }
}
