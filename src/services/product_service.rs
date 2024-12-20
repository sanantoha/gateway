use crate::error::Error;
use crate::models::product_models::{ProductRequest, ProductResponse};
use proto::product_client::ProductClient;
use tonic::transport::Channel;

mod proto {
    tonic::include_proto!("product");
}

#[derive(Debug, Clone)]
pub struct ProductService {
    client: ProductClient<Channel>,
}

impl ProductService {
    pub async fn new(product_endpoint: String) -> Result<Self, Error> {
        let channel = Channel::from_shared(product_endpoint)?.connect().await?;

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

        let product_response = ProductService::map_to_product(product);

        Ok(product_response)
    }

    pub async fn get_product_list(&self) -> Result<Vec<ProductResponse>, Error> {
        let request = tonic::Request::new(proto::Empty {});

        let mut client = self.client.clone();

        let response = client.get_product_list(request).await
            .map_err(|s| Error::GrpcStatus { input: "get_product_by_id failed".to_owned(), status: s})?;

        let product_list = response.into_inner();

        let response = product_list.products.into_iter().map(ProductService::map_to_product).collect();

        Ok(response)
    }

    pub async fn delete_product(&self, product_id: String) -> Result<bool, Error> {
        let request = tonic::Request::new(proto::DeleteProductRequest { id: product_id.clone() });

        let mut client = self.client.clone();

        let response = client.delete_product(request).await
            .map_err(|s| Error::GrpcStatus { input: format!("delete product with product_id = {} failed", product_id), status: s})?;

        let is_deleted = response.into_inner().is_deleted;

        Ok(is_deleted)
    }

    fn map_to_product(product: proto::ProductResponse) -> ProductResponse {
        ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            currency: product.current,
            price: product.price,
        }
    }
}