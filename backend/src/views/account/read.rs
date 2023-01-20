use crate::error::internal_server_error;
use crate::request::{did::Did, storage::Storage};
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::{account::Account, core::ToJson};
use log::trace;

#[axum_macros::debug_handler]
pub async fn read(Storage(stronghold): Storage, Did(did): Did) -> impl IntoResponse {
    // Read an identity if it exists using Stronghold as local storage.
    let account: Account = match Account::builder()
        .storage(stronghold)
        .load_identity(did)
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };
    trace!("account succesfully read");

    (
        StatusCode::OK,
        Json(account.document().to_json_value().unwrap()),
    )
}
