use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use tonic::transport::Channel;
use crate::error::Error;
use proto::order_client::OrderClient;
use crate::models::order_models::{OrderEntityResponse, OrderLineItems, OrderRequest};

mod proto {
    tonic::include_proto!("order");
}

#[derive(Debug, Clone)]
pub struct OrderService {
    client: OrderClient<Channel>
}

impl OrderService {
    pub async fn new(order_endpoint: String) -> Result<Self, Error> {
        let channel = Channel::from_shared(order_endpoint)?.connect().await?;

        let client = OrderClient::new(channel);

        Ok(Self { client })
    }

    pub async fn place_order(&self, order_request: OrderRequest) -> Result<String, Error> {
        let items: Vec<proto::OrderLineItems> = order_request.items.into_iter().map(|item| proto::OrderLineItems {
            sku_code: item.sku_code,
            price: item.price,
            quantity: item.quantity,
        }).collect();
        let request = tonic::Request::new(
            proto::OrderRequest {
                items,
            }
        );
        let mut client = self.client.clone();

        let response = client.place(request).await
            .map_err(|s| Error::GrpcStatus { input: "save order failed".to_owned(), status: s})?;

        let order_response = response.into_inner();

        Ok(order_response.order_number)
    }

    pub async fn get_order_list(&self) -> Result<Vec<OrderEntityResponse>, Error> {
        let request = tonic::Request::new(proto::Empty {});
        let mut client = self.client.clone();

        let response = client.get_order_list(request).await
            .map_err(|s| Error::GrpcStatus { input: "save order failed".to_owned(), status: s})?;

        let oer: Vec<OrderEntityResponse> = response.into_inner().orders.into_iter().map(|o| {
            OrderEntityResponse {
                order_id: o.order_id,
                order_number: o.order_number,
                created_at: o.created_at.and_then(OrderService::timestamp_to_datetime),
                items: o.items.into_iter().map(|item| {
                    OrderLineItems {
                        sku_code: item.sku_code,
                        quantity: item.quantity,
                        price: item.price,
                    }
                }).collect()
            }
        }).collect();

        Ok(oer)
    }

    fn timestamp_to_datetime(ts: Timestamp) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
    }
}