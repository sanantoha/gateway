use tonic::transport::Channel;
use crate::error::Error;
use proto::auth_client::AuthClient;
use crate::models::auth_models::{IsAdminResponse, LoginResponse, RegisterResponse};

mod proto {
    tonic::include_proto!("auth");
}

#[derive(Debug, Clone)]
pub struct AuthService {
    client: AuthClient<Channel>,
}

impl AuthService {
    pub async fn new(auth_endpoint: String) -> Result<Self, Error> {
        let channel = Channel::from_shared(auth_endpoint)?.connect().await?;

        let client = AuthClient::new(channel);

        Ok(Self { client })
    }

    pub async fn is_admin(&self, user_id: &str) -> Result<IsAdminResponse, Error> {
        let request = tonic::Request::new(proto::IsAdminRequest { 
            user_id: user_id.to_owned() 
        });

        let mut client = self.client.clone();

        let response = client.is_admin(request).await
            .map_err(|s| Error::GrpcStatus { input: "is_admin failed".to_owned(), status: s })?;

        Ok(IsAdminResponse {
            is_admin: response.into_inner().is_admin
        })
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<RegisterResponse, Error> {
        let request = tonic::Request::new(proto::RegisterRequest {
            email: email.to_owned(),
            password: password.to_owned(),
        });

        let mut client = self.client.clone();

        let response = client.register(request).await
            .map_err(|s| Error::GrpcStatus { input: "register failed".to_owned(), status: s })?;

        Ok(RegisterResponse {
            user_id: response.into_inner().user_id,
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, Error> {
        let request = tonic::Request::new(proto::LoginRequest {
            email: email.to_owned(),
            password: password.to_owned(),
            app_id: -1,
        });

        let mut client = self.client.clone();

        let response = client.login(request).await
            .map_err(|s| Error::GrpcStatus { input: "login failed".to_owned(), status: s })?;

        let token = response.into_inner().token;

        Ok(LoginResponse {
            email: email.to_owned(),
            token
        })
    }
}