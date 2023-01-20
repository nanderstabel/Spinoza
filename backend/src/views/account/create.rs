use crate::error::internal_server_error;
use crate::request::storage::Storage;
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::account::IdentitySetup;
use identity_iota::iota_core::IotaDID;
use identity_iota::{account::Account, core::ToJson};
use log::trace;

#[axum_macros::debug_handler]
pub async fn create(Storage(stronghold): Storage) -> impl IntoResponse {
    // Create a new identity using Stronghold as local storage.
    //
    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let account: Account = match Account::builder()
        .storage(stronghold)
        .create_identity(IdentitySetup::default())
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };
    trace!("account succesfully created");

    // Retrieve the did of the newly created identity.
    let iota_did: &IotaDID = account.did();

    (StatusCode::OK, Json(iota_did.to_json_value().unwrap()))
}
