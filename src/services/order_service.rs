use tonic::transport::Channel;
use crate::error::Error;
use proto::order_client::OrderClient;
use crate::models::order_models::OrderRequest;

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
        let items: Vec<proto::OrderLinItems> = order_request.items.into_iter().map(|item| proto::OrderLinItems {
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
}