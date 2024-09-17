use actix_web::{error, web, HttpResponse};
use log::{error, info};
use crate::error::Error;
use crate::models::auth_models::{ProductRequest, ProductResponse};
use crate::services::auth_service::AuthService;
use crate::services::product_service::ProductService;

pub async fn save_product(service: web::Data<ProductService>, body: web::Json<ProductRequest>) -> actix_web::Result<HttpResponse> {
    let request = body.into_inner();
    info!("save product request, name: {}, description: {}", request.name, request.description);

    match service.save_product(request).await  {
        Ok(product) => {
            info!("save product success, product.id: {}", product.id);
            Ok(HttpResponse::Ok().json(product))
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

pub async fn get_list_products(service: web::Data<ProductService>) -> actix_web::Result<HttpResponse> {
    info!("get_list_products request");

    match service.get_product_list().await {
        Ok(lst) => {
            Ok(HttpResponse::Ok().json(lst))
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