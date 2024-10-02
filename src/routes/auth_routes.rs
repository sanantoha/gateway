use actix_web::{web, HttpResponse};
use log::info;
use crate::models::auth_models::{LoginRequest, RegisterRequest};
use crate::routes::handle_result;
use crate::services::auth_service::AuthService;

pub async fn is_admin(service: web::Data<AuthService>, path: web::Path<(String,)>) -> actix_web::Result<HttpResponse> {
    let user_id = path.into_inner().0;
    info!("is_admin request: user_id={}", user_id);

    handle_result(service.is_admin(&user_id).await, |_| {
        info!("is_admin response for user_id: {}", user_id);
    })
}

pub async fn login(service: web::Data<AuthService>, body: web::Json<LoginRequest>) -> actix_web::Result<HttpResponse> {
    let login_body = body.into_inner();
    info!("login request: {}", login_body.email);

    handle_result(service.login(&login_body.email, &login_body.password).await, |response|{
        info!("login successfully for email: {}", response.email);
    })
}

pub async fn register(service: web::Data<AuthService>, body: web::Json<RegisterRequest>) -> actix_web::Result<HttpResponse> {
    let login_body = body.into_inner();
    info!("register request: {}", login_body.email);

    handle_result(service.register(&login_body.email, &login_body.password).await, |response| {
        info!("register successfully for email: {}, user_id: {}", login_body.email, response.user_id);
    })
}