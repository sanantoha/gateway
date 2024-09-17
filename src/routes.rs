use crate::error::Error;
use actix_web::{error, web, HttpResponse};
use log::error;
use serde::Serialize;

pub mod auth_routes;
mod product_routes;

use crate::middleware::jwt_validator::JwtValidator;
use crate::routes::auth_routes::{is_admin, login, register};
use crate::routes::product_routes::{get_list_products, save_product};

pub fn init_routes(cfg: &mut web::ServiceConfig, secret: String) {
    cfg.service(
        web::resource("/auth/is_admin/{id}")
            .wrap(JwtValidator { secret: secret.clone() })
            .route(web::get().to(is_admin))
    )
    .service(login)
    .service(register)
    .service(
        web::resource("/products")
            .wrap(JwtValidator { secret: secret.clone() })
            .route(web::post().to(save_product))
            .route(web::get().to(get_list_products))
    );
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
