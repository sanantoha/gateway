use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub items: Vec<OrderLineItems>
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_number: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLineItems {
    pub sku_code: String,
    pub price: i64,
    pub quantity: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderEntityResponse {
    pub order_id: i64,
    pub order_number: String,
    pub created_at: Option<DateTime<Utc>>,
    pub items: Vec<OrderLineItems>
}