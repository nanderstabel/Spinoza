use axum::{async_trait, extract::FromRequestParts, http};
use derive_more::Deref;
use http::{request::Parts, StatusCode};
use identity_iota::iota_core::IotaDID;
use serde::Deserialize;

// Wrapper for [`IotaDID`] which allows for extraction from an http request when a request handler is called.
#[derive(Deserialize, Deref)]
pub struct Did(pub IotaDID);

#[async_trait]
impl<S> FromRequestParts<S> for Did
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the did from the request headers.
        let did = parts
            .headers
            .get("did")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        Ok(Did(IotaDID::parse(did).unwrap()))
    }
}

// Wrapper for a pair of [`IotaDID`] which allows for extraction from an http request when a request handler is called.
#[derive(Deserialize)]
pub struct DidPair(pub IotaDID, pub IotaDID);

#[async_trait]
impl<S> FromRequestParts<S> for DidPair
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the issuer did from the request headers.
        let issuer_did = parts
            .headers
            .get("issuer_did")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        // Extract the holder did from the request headers.
        let holder_did = parts
            .headers
            .get("holder_did")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        Ok(DidPair(
            IotaDID::parse(issuer_did).unwrap(),
            IotaDID::parse(holder_did).unwrap(),
        ))
    }
}
