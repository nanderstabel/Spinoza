use crate::error::internal_server_error;
use crate::request::did::DidPair;
use crate::request::storage::Storage;
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use identity_iota::client::{CredentialValidationOptions, CredentialValidator, FailFast};
use identity_iota::core::FromJson;
use identity_iota::core::Url;
use identity_iota::credential::{Credential, CredentialBuilder, Subject};
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DID;
use identity_iota::{account::Account, core::ToJson};
use log::trace;
use serde_json::json;

#[axum_macros::debug_handler]
pub async fn create(
    Storage(stronghold): Storage,
    DidPair(issuer_did, holder_did): DidPair,
) -> impl IntoResponse {
    // Create a new credential using Stronghold as local storage and using both an issuer- and holder did.

    // Load the identity of the issuer.
    let issuer: Account = match Account::builder()
        .storage(stronghold)
        .load_identity(issuer_did)
        .await
    {
        Ok(account) => account,
        Err(err) => return internal_server_error(err.into()),
    };
    trace!("issuer account succesfully read");

    // Create a credential subject indicating the lifespan by Baruch.
    let subject: Subject = Subject::from_json_value(json!({
      "id": holder_did,
      "name": "Baruch",
      "lifespan": {
        "born": "24-11-1632",
        "died": "21-02-1677",
      }
    }))
    .unwrap();

    // Build credential using subject above and issuer.
    let mut credential: Credential = CredentialBuilder::default()
        .id(Url::parse("https://example.edu/birth-certificate/baruch-de-spinoza").unwrap())
        .issuer(Url::parse(issuer.did().as_str()).unwrap())
        .type_("LifeSpan")
        .subject(subject)
        .build()
        .unwrap();

    // Sign the Credential with the issuer's verification method.
    issuer
        .sign("#issuerKey", &mut credential, ProofOptions::default())
        .await
        .unwrap();

    trace!("Credential JSON > {:#}", credential);

    // Before sending this credential to the holder the issuer wants to validate that some properties
    // of the credential satisfy their expectations.

    // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
    // that the issuance date is not in the future and that the expiration date is not in the past:
    CredentialValidator::validate(
        &credential,
        &issuer.document(),
        &CredentialValidationOptions::default(),
        FailFast::FirstError,
    )
    .unwrap();

    trace!("credential successfully validated");

    // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // The credential is then serialized to JSON and transmitted to the subject in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.

    (StatusCode::OK, Json(credential.to_json_value().unwrap()))
}
