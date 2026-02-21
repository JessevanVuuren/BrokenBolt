use std::sync::Arc;

use crate::{
    Kraken, TradeHistoryBody,
    fetch::{error::AuthFetchError, types::Trade},
};

#[derive(Debug, Clone)]
pub struct Trades {
    pub trades: Vec<Trade>,
    kraken: Arc<Kraken>,
}

impl Trades {
    pub async fn new(kraken: Arc<Kraken>) -> Result<Self, AuthFetchError> {
        let body = TradeHistoryBody::default();
        let trades = kraken.get_trades_history(&body).await?;

        Ok(Self {
            trades,
            kraken,
        })
    }

    pub async fn update_trades(&mut self) {
        let body = TradeHistoryBody::default();
        if let Ok(trades) = self.kraken.get_trades_history(&body).await {
            self.trades = trades;
        }
    }
}
