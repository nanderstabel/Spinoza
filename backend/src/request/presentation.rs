use axum::{
    async_trait,
    body::Bytes,
    extract::FromRequest,
    http::{self, Request},
};
use http::StatusCode;
use identity_iota::{core::FromJson, credential::Presentation};
use serde_json::Value;
use tower_http::BoxError;

// Wrapper for [`Presentation`] which allows for extraction from an http request when a request handler is called.
pub struct Present(pub Presentation);

#[async_trait]
impl<S, B> FromRequest<S, B> for Present
where
    // these bounds are required by `async_trait`
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let value: Value = serde_json::from_slice(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

        let presentation: Presentation =
            Presentation::from_json_value(value).map_err(|_| StatusCode::BAD_REQUEST)?;

        Ok(Present(presentation))
    }
}
