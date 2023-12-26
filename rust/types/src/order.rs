use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Option<Decimal>,
    pub executed_quantity: Decimal,
    pub quote_quantity: Option<Decimal>,
    pub executed_quote_quantity: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub executed_quote_quantity: Decimal,
    pub price: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub post_only: bool,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "orderType")]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum SelfTradePrevention {
    RejectTaker,
    RejectMaker,
    RejectBoth,
    Allow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Cancelled,
    Expired,
    Filled,
    New,
    PartiallyFilled,
    Triggered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOrderPayload {
    pub client_id: Option<u32>,
    pub order_type: OrderType,
    pub post_only: Option<bool>,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub quote_quantity: Option<Decimal>,
    pub self_trade_prevention: Option<SelfTradePrevention>,
    pub side: Side,
    pub symbol: String,
    pub time_in_force: Option<TimeInForce>,
    pub trigger_price: Option<Decimal>,
}
