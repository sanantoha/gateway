use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::{ok, LocalBoxFuture, Ready};
use log::{error, info};
use reqwest::Client;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct MetricsMiddleware {
    pub influxdb_client: Arc<Client>,
    pub token: Arc<String>,
    pub url: Arc<String>,
    pub org: Arc<String>,
    pub bucket: Arc<String>,
}

impl MetricsMiddleware {
    pub fn new(influxdb_client: Arc<Client>, token: Arc<String>, url: Arc<String>, org: Arc<String>, bucket: Arc<String>) -> Self {
        MetricsMiddleware {
            influxdb_client,
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
            token: Arc::clone(&self.token),
            url: Arc::clone(&self.url),
            org: Arc::clone(&self.org),
            bucket: Arc::clone(&self.bucket)
        })
    }
}

// Middleware logic
pub struct MetricsMiddlewareService<S> {
    service: S,
    influxdb_client: Arc<Client>,
    token: Arc<String>,
    url: Arc<String>,
    org: Arc<String>,
    bucket: Arc<String>,
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
        let method = req.method().to_string();
        let path = req.path().to_string();
        let start = Instant::now();
        let influxdb_client = Arc::clone(&self.influxdb_client);
        let token = Arc::clone(&self.token);
        let org = Arc::clone(&self.org);
        let bucket = Arc::clone(&self.bucket);
        let url = Arc::clone(&self.url);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed().as_millis();
            let status_code = res.response().status();

            let mut metric_name = "http_requests_gateway";
            if status_code.is_server_error() || status_code.is_client_error() {
                metric_name = "error_metric";
            }

            // Publish metrics to InfluxDB asynchronously (without waiting)
            tokio::spawn(async move {
                publish_metric(influxdb_client, metric_name, &method, &path, token.as_str(), url.as_str(), org.as_str(), bucket.as_str(), status_code.as_u16(), duration).await;
            });

            Ok(res)
        })
    }
}

pub async fn publish_metric(client: Arc<Client>, metric_name: &str, method: &str, path: &str, token: &str, url: &str, org: &str, bucket: &str, status_code: u16, duration: u128) {

    let data = format!("{},method={},request_path={},status={} response_time={}", metric_name, method, path, status_code, duration);
    // let data = format!("requests,name={} count=1", method);

    let write_url = format!("{}/api/v2/write?org={}&bucket={}&precision=ms", url, org, bucket);

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