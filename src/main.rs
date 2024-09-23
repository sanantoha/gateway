extern crate core;

mod error;
mod routes;
mod services;
mod models;
mod middleware;

use std::env;
use actix_web::{web, App, HttpServer};
use routes::init_routes;
use crate::services::auth_service::AuthService;
use crate::error::Error;
use crate::services::product_service::ProductService;
use actix_cors::Cors;
use crate::services::order_service::OrderService;

const SECRET_NAME: &str = "AUTH_SECRET";

const GATEWAY_ADDR: &str = "GATEWAY_ADDRS";

const GATEWAY_CORS_ORIGIN: &str = "GATEWAY_CORS_ORIGIN";

const AUTH_ENDPOINT: &str = "AUTH_ENDPOINT";

const PRODUCT_ENDPOINT: &str = "PRODUCT_ENDPOINT";

const ORDER_ENDPOINT: &str = "ORDER_ENDPOINT";

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or("info".to_owned()));
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let secret = env::var(SECRET_NAME)
        .map_err(|e| Error::Var { input: SECRET_NAME, source: e })?;

    let addrs = env::var(GATEWAY_ADDR)
        .map_err(|e| Error::Var { input: GATEWAY_ADDR, source: e })?;

    let cors_origin = env::var(GATEWAY_CORS_ORIGIN)
        .map_err(|e| Error::Var { input: GATEWAY_CORS_ORIGIN, source: e })?;

    let auth_endpoint = env::var(AUTH_ENDPOINT)
        .map_err(|e| Error::Var { input: AUTH_ENDPOINT, source: e })?;

    let product_endpoint = env::var(PRODUCT_ENDPOINT)
        .map_err(|e| Error::Var { input: PRODUCT_ENDPOINT, source: e })?;

    let order_endpoint = env::var(ORDER_ENDPOINT)
        .map_err(|e| Error::Var { input: ORDER_ENDPOINT, source: e })?;

    let auth_service = AuthService::new(auth_endpoint).await?;

    let product_service = ProductService::new(product_endpoint).await?;

    let order_service = OrderService::new(order_endpoint).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&cors_origin)
                    .allow_any_method()
                    .allow_any_header())
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(product_service.clone()))
            .app_data(web::Data::new(order_service.clone()))
            .configure(|cfg| init_routes(cfg, secret.clone()))
    })
    .bind(addrs)?
    .run()
    .await?;

    Ok(())
}
