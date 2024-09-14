use tonic::transport::Channel;
use crate::error::Error;
use proto::product_client::ProductClient;
use crate::models::auth_models::{ProductRequest, ProductResponse};

mod proto {
    tonic::include_proto!("product");
}

#[derive(Debug, Clone)]
pub struct ProductService {
    client: ProductClient<Channel>,
}

impl ProductService {
    pub async fn new() -> Result<Self, Error> {
        // PRODUCT_PORT=55005
        let channel = Channel::from_static("http://[::1]:55005")
            .connect()
            .await?;

        let client = ProductClient::new(channel);

        Ok(Self { client })
    }

    pub async fn save_product(&self, product_request: ProductRequest) -> Result<ProductResponse, Error> {
        let request = tonic::Request::new(proto::ProductRequest {
            name: product_request.name,
            description: product_request.description,
            currency: product_request.currency,
            price: product_request.price,
        });

        let mut client = self.client.clone();

        let response = client.save(request).await
            .map_err(|s| Error::GrpcStatus { input: "save product failed".to_owned(), status: s})?;

        let product = response.into_inner();

        let product_response = ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            currency: product.current,
            price: product.price,
        };

        Ok(product_response)
    }
}