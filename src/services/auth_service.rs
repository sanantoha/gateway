use tonic::transport::Channel;
use crate::error::Error;
use proto::auth_client::AuthClient;

mod proto {
    tonic::include_proto!("auth");
}

#[derive(Debug, Clone)]
pub struct AuthService {
    client: AuthClient<Channel>,
}

impl AuthService {
    pub async fn new() -> Result<Self, Error> {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;

        let client = AuthClient::new(channel);

        Ok(Self { client })
    }

    pub async fn is_admin(&self, user_id: &str) -> Result<bool, Error> {
        let request = tonic::Request::new(proto::IsAdminRequest { 
            user_id: user_id.to_owned() 
        });

        let mut client = self.client.clone();

        let response = client.is_admin(request).await
            .map_err(|s| Error::GrpcStatus { input: "is_admin failed".to_owned(), status: s })?;

        Ok(response.into_inner().is_admin)
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<String, Error> {
        let request = tonic::Request::new(proto::RegisterRequest {
            email: email.to_owned(),
            password: password.to_owned(),
        });

        let mut client = self.client.clone();

        let response = client.register(request).await
            .map_err(|s| Error::GrpcStatus { input: "register failed".to_owned(), status: s })?;

        Ok(response.into_inner().user_id)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, Error> {
        let request = tonic::Request::new(proto::LoginRequest {
            email: email.to_owned(),
            password: password.to_owned(),
            app_id: -1,
        });

        let mut client = self.client.clone();

        let response = client.login(request).await
            .map_err(|s| Error::GrpcStatus { input: "login failed".to_owned(), status: s })?;

        Ok(response.into_inner().token)
    }
}