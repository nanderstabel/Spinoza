use crate::{
    error::internal_server_error,
    request::{did::DidPair, presentation::Present, storage::Storage, vc::VC},
};
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::{
    account::Account,
    client::{FailFast, PresentationValidationOptions, Resolver},
    core::{ToJson, Url},
    credential::{Presentation, PresentationBuilder},
    crypto::ProofOptions,
};
use log::trace;
use serde_json::json;

#[axum_macros::debug_handler]
pub async fn present(
    Storage(stronghold): Storage,
    DidPair(_issuer_did, holder_did): DidPair,
    VC(credential): VC,
) -> impl IntoResponse {
    // Load the identity of the holder.
    let holder: Account = match Account::builder()
        .storage(stronghold)
        .load_identity(holder_did)
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };
    dbg!(&credential);
    // Create an unsigned Presentation from the previously issued Verifiable Credential.
    let mut presentation: Presentation = match PresentationBuilder::default()
        .holder(Url::parse(holder.did().as_ref()).unwrap())
        .credential(credential)
        .build()
    {
        Ok(presentation) => presentation,
        Err(err) => return internal_server_error(err.into()),
    };
    dbg!(&presentation);
    trace!("presentation succesfully created");

    // Sign the verifiable presentation using the holder's verification method.
    holder
        .sign("issuerKey", &mut presentation, ProofOptions::default())
        .await
        .unwrap();

    (StatusCode::OK, Json(presentation.to_json_value().unwrap()))
}

#[axum_macros::debug_handler]
pub async fn verify(
    // Storage(stronghold): Storage,
    // DidPair(_issuer_did, holder_did): DidPair,
    Present(presentation): Present,
) -> impl IntoResponse {
    // Validate the presentation and all the credentials included in it.
    let resolver: Resolver = Resolver::new().await.unwrap();
    resolver
        .verify_presentation(
            &presentation,
            &PresentationValidationOptions::default(),
            FailFast::FirstError,
            None,
            None,
        )
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(json!("presentation: succesfully verified")),
    )
}
