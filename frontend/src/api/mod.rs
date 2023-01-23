use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

const BASE_URL: &str = include_str!("api_base_uri.txt");

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub did: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     pub id: u32,
//     pub username: String,
//     pub token: String,
// }

pub async fn create_account(stronghold: String, password: String) -> AuthResponse {
    let did = Request::post(&format!("{}/account/create", BASE_URL))
        .header("Content-Type", "application/json")
        .header("stronghold", stronghold.as_str())
        .header("password", password.as_str())
        .send()
        .await
        .unwrap()
        .json::<String>()
        .await
        .unwrap();

    AuthResponse { did }
}
