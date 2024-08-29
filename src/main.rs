
mod error;
mod routes;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use routes::init_routes;
use crate::services::auth_service::AuthService;
use crate::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").unwrap_or("info".to_owned()));
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let auth_service = AuthService::new().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .configure(init_routes)
    })
    .bind("0.0.0.0:8085")?
    .run()
    .await?;

    Ok(())
}
