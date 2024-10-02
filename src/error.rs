//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),

    #[error("gRPC failure {input}")]
    GrpcStatus {
        input: String,
        #[source]
        status: tonic::Status
    },

    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Can not parse variable: {input}")]
    Var {
        input: &'static str,
        #[source]
        source: std::env::VarError,
    },

    #[error(transparent)]
    InvalidUrl(#[from] tonic::codegen::http::uri::InvalidUri),

    #[error("toml file read error occurred: {0}")]
    InfluxdbHttpRequest(#[from] reqwest::Error),
}