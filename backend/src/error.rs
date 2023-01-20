use axum::Json;
use http::StatusCode;
use log::error;
use serde_json::Value;

pub fn internal_server_error(err: anyhow::Error) -> (StatusCode, Json<Value>) {
    error!("Internal server error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(format!("Unhandled internal error: {}", err).into()),
    )
}
