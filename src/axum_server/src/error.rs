use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum StartError {
    ParseSocketAddr(core::net::AddrParseError),
    TokioListenerBind(std::io::Error),
    ServeHttpService(std::io::Error),
}

#[derive(Debug)]
pub enum ApiError {
    GrpcConnect(tonic::transport::Error),
    GrpcRequest(tonic::Status),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message): (StatusCode, &str) = match self {
            ApiError::GrpcConnect(error) => {
                error!("could not connect to grpc server, because: {:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not connect to grpc server",
                )
            }
            ApiError::GrpcRequest(status) => {
                error!("could not do grpc request: {:?}", status);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not do grpc request",
                )
            }
        };

        let value = json!({ "message": message });

        (status_code, Json(value)).into_response()
    }
}
