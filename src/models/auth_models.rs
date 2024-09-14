use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct IsAdminResponse {
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub email: String,
    pub token: String
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: String
}

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