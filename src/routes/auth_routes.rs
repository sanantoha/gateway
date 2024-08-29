use actix_web::web::service;
use actix_web::{get, post, web, HttpResponse, Responder, error};
use log::{info, error};
use crate::error::Error;
use crate::models::auth_models::{IsAdminResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::services::auth_service::AuthService;


#[get("/auth/is_admin/{id}")]
pub async fn is_admin(service: web::Data<AuthService>, path: web::Path<(String,)>) -> actix_web::Result<HttpResponse> {
    let user_id = path.into_inner().0;
    info!("is_admin request: user_id={}", user_id);

    match service.is_admin(&user_id).await {
        Ok(is_admin) => {      
            Ok(HttpResponse::Ok().json(IsAdminResponse { 
                is_admin
            }))
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

#[post("/auth/login")]
pub async fn login(service: web::Data<AuthService>, body: web::Json<LoginRequest>) -> actix_web::Result<HttpResponse> {
    let login_body = body.into_inner();
    info!("login request: {}", login_body.email);

    match service.login(&login_body.email, &login_body.password).await {
        Ok(token) => {
            Ok(HttpResponse::Ok().json(LoginResponse {
                email: login_body.email,
                token
            }))
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

#[post("/auth/register")]
pub async fn register(service: web::Data<AuthService>, body: web::Json<RegisterRequest>) -> actix_web::Result<HttpResponse> {
    let login_body = body.into_inner();
    info!("register request: {}", login_body.email);

    match service.register(&login_body.email, &login_body.password).await {
        Ok(user_id) => {      
            Ok(HttpResponse::Ok().json(RegisterResponse {
                user_id,
            }))
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