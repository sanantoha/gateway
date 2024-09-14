
use actix_web::web;
pub mod auth_routes;
mod product_routes;

use crate::routes::auth_routes::{is_admin, login, register};
use crate::middleware::jwt_validator::JwtValidator;
use crate::routes::product_routes::save_product;

pub fn init_routes(cfg: &mut web::ServiceConfig, secret: String) {
    cfg.service(
        web::resource("/auth/is_admin/{id}")
            .wrap(JwtValidator { secret: secret.clone() })
            .route(web::get().to(is_admin))
    )
    .service(login)
    .service(register)
    .service(
        web::resource("/product")
            .wrap(JwtValidator { secret: secret.clone() })
            .route(web::post().to(save_product))
    );
}

