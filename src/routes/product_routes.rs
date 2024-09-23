use actix_web::{web, HttpResponse};
use log::info;
use crate::models::product_models::ProductRequest;
use crate::routes::handle_result;
use crate::services::product_service::ProductService;

pub async fn save_product(service: web::Data<ProductService>, body: web::Json<ProductRequest>) -> actix_web::Result<HttpResponse> {
    let request = body.into_inner();
    info!("save product request, name: {}, description: {}", request.name, request.description);

    handle_result(service.save_product(request).await, |product| {
        info!("save product success, product.id: {}", product.id);
    })
}

pub async fn get_list_products(service: web::Data<ProductService>) -> actix_web::Result<HttpResponse> {
    info!("get_list_products request");
    handle_result(service.get_product_list().await, |products| {
        info!("get_list_products response, products: {}", products.len());
    })
}