use core::fmt;
use std::default;

use nestify::nest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Limit,
    #[default]
    Market,
    Iceberg,
    StopLoss,
    TakeProfit,
    TrailingStop,
    StopLossLimit,
    SettlePosition,
    TakeProfitLimit,
    TrailingStopLimit,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StpType {
    #[default]
    CancelNewest,
    CancelOldest,
    CancelBoth,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    GTC, // Good-'til-cancelled
    IOC, // Immediate-or-cancel
    GTD, // Good-'til-date
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    #[default]
    BUY, // Good-'til-cancelled
    SELL, // Immediate-or-cancel
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddOrder {
    pub ordertype: OrderType,
    #[serde(rename = "type")]
    pub type_field: Side,
    pub volume: String,
    pub pair: String,
    
    // optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userref")]
    pub user_ref: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "displayvol")]
    pub display_vol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stptype")]
    pub stp_type: Option<StpType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oflags")]
    pub of_lags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeinforce")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "starttm")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "expiretm")]
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
