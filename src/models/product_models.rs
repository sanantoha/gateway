use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProductRequest {
    pub name: String,
    pub description: String,
    pub currency: String,
    pub price: i64
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub currency: String,
    pub price: i64
}

#[derive(Serialize)]
pub struct DeleteProductResponse {
    pub is_deleted: bool
}