use std::sync::Arc;
use crate::error::Error as AppError;
use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};


// Middleware structure
pub struct JwtValidator {
    pub secret: Arc<String>,
}

impl JwtValidator {
    pub fn new(secret: Arc<String>) -> Self {
        JwtValidator { secret }
    }
}

// Implement `Transform` for middleware
impl<S, B> Transform<S, ServiceRequest> for JwtValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtValidatorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtValidatorMiddleware {
            service,
            secret: Arc::clone(&self.secret),
        })
    }
}

// Middleware logic
pub struct JwtValidatorMiddleware<S> {
    service: S,
    secret: Arc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(String::from);

        if let Some(token) = token {
            match validate_jwt(&token, self.secret.as_str()) {
                Ok(_) => {
                    Box::pin(self.service.call(req))
                }
                Err(_) => {
                    // Invalid token, return Unauthorized response
                    Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
                }
            }
        } else {
            // No token, return Unauthorized response
            Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
        }
    }
}

#[warn(private_interfaces)]
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize, // Expiration time (as UTC timestamp)
}

fn validate_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data)
}