//! Main Crate Error



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),

    #[error("gRPC failure {input}")]
    GrpcStatus {
        input: String,
        #[source]
        status: tonic::Status
    }
}