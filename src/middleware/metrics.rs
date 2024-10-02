use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, LocalBoxFuture, Ready};
use reqwest::Client;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use log::{error, info};
use actix_web::Error;

pub struct MetricsMiddleware {
    pub influxdb_client: Arc<Client>,
    pub token: String,
    pub url: String,
    pub org: String,
    pub bucket: String,
}

impl MetricsMiddleware {
    pub fn new(influxdb_client: Client, token: String, url: String, org: String, bucket: String) -> Self {
        MetricsMiddleware {
            influxdb_client: Arc::new(influxdb_client),
            token,
            url,
            org,
            bucket,
        }
    }
}

// Implement `Transform` for middleware
impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MetricsMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddlewareService {
            service,
            influxdb_client: Arc::clone(&self.influxdb_client),
            token: self.token.clone(),
            url: self.url.clone(),
            org: self.org.clone(),
            bucket: self.bucket.clone()
        })
    }
}

// Middleware logic
pub struct MetricsMiddlewareService<S> {
    service: S,
    influxdb_client: Arc<Client>,
    token: String,
    url: String,
    org: String,
    bucket: String,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header

        let method = req.method().to_string();
        let path = req.path().to_string();
        let start = Instant::now();
        let influxdb_client = Arc::clone(&self.influxdb_client);
        let token = self.token.clone();
        let org = self.org.clone();
        let bucket = self.bucket.clone();
        let url = self.url.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed().as_millis();

            // Publish metrics to InfluxDB asynchronously (without waiting)
            tokio::spawn(async move {
                publish_metric(influxdb_client, "http_requests_gateway", &method, &path, &token, &url, &org, &bucket, duration).await;
            });

            Ok(res)
        })
    }
}

async fn publish_metric(client: Arc<Client>, metric_name: &str, method: &str, path: &str, token: &str, url: &str, org: &str, bucket: &str, duration: u128) {

    let data = format!("{},method={},request_path={} response_time={}", metric_name, method, path, duration);
    // let data = format!("requests,name={} count=1", method);

    let write_url = format!("{}/api/v2/write?org={}&bucket={}&precision=ns", url, org, bucket);

    // Send the request
    let response = client
        .post(&write_url)
        .header("Authorization", format!("Token {}", token))
        .header("Content-Type", "text/plain")
        .body(data)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                info!("Data point sent successfully!");
            } else {
                error!("Failed to send data point. Status: {}", res.status());
            }
        }
        Err(e) => {
            error!("Failed to send data point. Error: {}", e);
        }
    }
}