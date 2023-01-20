use crate::error::internal_server_error;
use crate::request::{did::Did, storage::Storage};
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::account::Account;
use log::trace;
use serde_json::json;

#[axum_macros::debug_handler]
pub async fn delete(Storage(stronghold): Storage, Did(did): Did) -> impl IntoResponse {
    // Delete an identity if it exists using Stronghold as local storage.
    let account: Account = match Account::builder()
        .storage(stronghold)
        .load_identity(did)
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };

    account.delete_identity().await.unwrap();
    trace!("account succesfully deleted");

    (StatusCode::OK, Json(json!("account: succesfully deleted")))
}
