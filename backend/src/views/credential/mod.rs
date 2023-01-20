use axum::{routing::post, Router};

pub mod create;
pub mod update;

// Redirects to CRUD operations for [`Credential`].
pub fn credential_views_factory() -> Router<()> {
    Router::new()
        .route("/create", post(create::create))
        // .route("/read", post(read::read))
        .route("/present", post(update::present))
        .route("/verify", post(update::verify))
    // .route("/delete", post(delete::delete))
}
