use actix_web::{web, HttpResponse};
use log::info;
use crate::models::order_models::OrderRequest;
use crate::routes::handle_result;
use crate::services::order_service::OrderService;
use itertools::Itertools;

pub async fn place_order(service: web::Data<OrderService>, body: web::Json<OrderRequest>) -> actix_web::Result<HttpResponse> {
    let request = body.into_inner();
    let sku_codes = request.items.iter()
        .map(|x| &x.sku_code).join(",");
    info!("save product request, sku_codes {}", sku_codes);

    handle_result(service.place_order(request).await, |order_number| {
        info!("save order success, order_number: {}", order_number);
    })
}

pub async fn get_order_list(service: web::Data<OrderService>) -> actix_web::Result<HttpResponse> {
    info!("get_list_orders request");

    handle_result(service.get_order_list().await, |oer| {
        info!("get order list return {} orders", oer.len());
    })
}

pub async fn delete_order(service: web::Data<OrderService>, id: web::Path<i64>) -> actix_web::Result<HttpResponse> {
    let order_id = id.into_inner();
    info!("delete_order request order_id = {}", order_id);

    handle_result(service.delete_order(order_id).await, |oer| {
        let mut deleted_msg = "is deleted";
        if !oer {
           deleted_msg = "is not deleted"
        }
        info!("order_id = {} {}", order_id, deleted_msg);
    })
}