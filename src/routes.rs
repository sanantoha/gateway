use crate::error::Error;
use actix_web::{error, web, HttpResponse};
use log::error;
use reqwest::Client;
use serde::Serialize;

pub mod auth_routes;
mod product_routes;
mod order_routes;

use crate::middleware::jwt_validator::JwtValidator;
use crate::middleware::metrics::MetricsMiddleware;
use crate::routes::auth_routes::{is_admin, login, register};
use crate::routes::order_routes::{delete_order, get_order_list, place_order};
use crate::routes::product_routes::{delete_product, get_list_products, save_product};

pub fn init_routes(cfg: &mut web::ServiceConfig, secret: String, influxdb_client: Client, token: String, url: String, org: String, bucket: String) {
    cfg.service(
        web::resource("/auth/is_admin/{id}")
            .wrap(JwtValidator::new(secret.clone()))
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::get().to(is_admin))
    )
    .service(
        web::resource("/auth/login")
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::post().to(login))
    )
    .service(
        web::resource("/auth/register")
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::post().to(register))
    )
    .service(
        web::resource("/products")
            .wrap(JwtValidator::new(secret.clone()))
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::post().to(save_product))
            .route(web::get().to(get_list_products))
    )
    .service(
        web::resource("/products/{id}")
            .wrap(JwtValidator::new(secret.clone()))
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::delete().to(delete_product))
    )
    .service(
        web::resource("/orders")
            .wrap(JwtValidator::new(secret.clone()))
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::post().to(place_order))
            .route(web::get().to(get_order_list))
    )
    .service(
        web::resource("/orders/{id}")
            .wrap(JwtValidator::new(secret.clone()))
            .wrap(MetricsMiddleware::new(influxdb_client.clone(), token.clone(), url.clone(), org.clone(), bucket.clone()))
            .route(web::delete().to(delete_order))
    )
    .default_service(web::to(HttpResponse::NotFound))
    ;
}

pub fn handle_result<T, F>(res: Result<T, Error>, log_f: F) -> actix_web::Result<HttpResponse>
where
    T: Serialize,
    F: Fn(&T),
{
    match res {
        Ok(t) => {
            log_f(&t);
            Ok(HttpResponse::Ok().json(t))
        },
        Err(Error::GrpcStatus { input, status }) => {
            error!("{}, {}", input, status);
            Err(error::ErrorInternalServerError(format!("{}, {}", input, status)))
        },
        Err(e) => {
            error!("{}", e);
            Err(error::ErrorInternalServerError(format!("{}", e)))
        }
    }
}
