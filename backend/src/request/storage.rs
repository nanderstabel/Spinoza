use axum::{async_trait, extract::FromRequestParts, http};
use http::{request::Parts, StatusCode};
use identity_iota::account_storage::Stronghold;
use log::trace;

// Wrapper for [`Stronghold`] which allows for extraction from an http request when a request handler is called.
pub struct Storage(pub Stronghold);

#[async_trait]
impl<S> FromRequestParts<S> for Storage
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the path for the stronghold file from the request headers.
        let stronghold_path = parts
            .headers
            .get("stronghold")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        // Extract the password for the stronghold file from the request headers.
        let password = parts
            .headers
            .get("password")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        // Obtain the Stronghold object.
        let stronghold: Stronghold =
            match Stronghold::new(&stronghold_path, password.to_owned(), None).await {
                Ok(stronghold) => stronghold,
                Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized")),
            };
        trace!("stronghold succesfully created");

        Ok(Storage(stronghold))
    }
}
