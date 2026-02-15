use core::fmt;

use nestify::nest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TradeHistoryBody {
    pub trade_type: TradeType,
    pub trades: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ofs: Option<i64>,
    pub without_count: bool,
    pub consolidate_taker: bool,
    pub ledgers: bool,
    pub rebase_multiplier: BalanceType,
}

impl Default for TradeHistoryBody {
    fn default() -> Self {
        TradeHistoryBody {
            trade_type: TradeType::default(),
            trades: false,
            start: None,
            end: None,
            ofs: None,
            without_count: false,
            consolidate_taker: true,
            ledgers: false,
            rebase_multiplier: BalanceType::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeType {
    #[default]
    All,
    AnyPosition,
    ClosedPosition,
    ClosingPosition,
    NoPosition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BalanceType {
    #[default]
    Rebased,
    Base,
}

impl fmt::Display for BalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BalanceType::Base => write!(f, "base"),
            BalanceType::Rebased => write!(f, "rebased"),
        }
    }
}
