use crate::types::types::{Params, SubRequest};

pub struct Channel {
    pairs: Vec<String>,
    subscribe: SubRequest,
}

impl Channel {
    pub fn new(channel: &str, pairs: Vec<&str>) -> Self {
        let pairs: Vec<String> = pairs.iter().map(|p| p.to_string()).collect();
        let subscribe = SubRequest {
            method: "subscribe".into(),
            params: Params {
                channel: channel.into(),
                symbol: pairs.clone(),
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
