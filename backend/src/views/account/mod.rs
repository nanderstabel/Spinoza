use axum::{routing::post, Router};

pub mod create;
pub mod delete;
pub mod read;
pub mod update;

// Redirects to CRUD operations for [`Account`].
pub fn account_views_factory() -> Router<()> {
    Router::new()
        .route("/create", post(create::create))
        .route("/read", post(read::read))
        .route("/update", post(update::update))
        .route("/delete", post(delete::delete))
}
