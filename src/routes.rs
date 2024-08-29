
use actix_web::web;
pub mod auth_routes;
use crate::routes::auth_routes::{is_admin, login, register};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(is_admin)
        .service(login)
        .service(register);
}

