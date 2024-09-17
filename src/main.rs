extern crate core;

mod error;
mod routes;
mod services;
mod models;
mod middleware;

use std::env;
use actix_web::{web, App, HttpServer};
use log::info;
use routes::init_routes;
use crate::services::auth_service::AuthService;
use crate::error::Error;
use crate::services::product_service::ProductService;
use actix_cors::Cors;

const SECRET_NAME: &str = "AUTH_SECRET";

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or("info".to_owned()));
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let secret = env::var(SECRET_NAME)
        .map_err(|e| Error::Var { input: SECRET_NAME, source: e })?;

    let auth_service = AuthService::new().await?;

    let product_service = ProductService::new().await?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allow_any_method()
                    .allow_any_header())
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(product_service.clone()))
            .configure(|cfg| init_routes(cfg, secret.clone()))
    })
    .bind("0.0.0.0:8085")?
    .run()
    .await?;

    Ok(())
}
