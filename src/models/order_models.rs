use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub items: Vec<OrderLineItems>
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_number: String
}

#[derive(Debug, Deserialize)]
pub struct OrderLineItems {
    pub sku_code: String,
    pub price: i64,
    pub quantity: i64,
}