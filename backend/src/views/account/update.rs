use crate::{
    error::internal_server_error,
    request::{did::Did, storage::Storage},
};
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::{
    account::{Account, MethodContent},
    core::ToJson,
};
use log::trace;

#[axum_macros::debug_handler]
pub async fn update(Storage(stronghold): Storage, Did(did): Did) -> impl IntoResponse {
    // Update an identity if it exists using Stronghold as local storage.
    let mut account: Account = match Account::builder()
        .storage(stronghold)
        .load_identity(did)
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };

    // Add verification method to the issuer. In this case always the same method is used.
    account
        .update_identity()
        .create_method()
        .content(MethodContent::GenerateEd25519)
        .fragment("issuerKey")
        .apply()
        .await
        .unwrap();
    trace!("account succesfully updated");

    (
        StatusCode::OK,
        Json(account.document().to_json_value().unwrap()),
    )
}
