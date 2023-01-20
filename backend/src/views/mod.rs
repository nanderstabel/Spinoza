mod account;
mod credential;

use account::account_views_factory;
use axum::Router;
use credential::credential_views_factory;

// Redirect to either `account` or `credential`.
pub fn views_factory() -> Router<()> {
    Router::new()
        .nest("/account", account_views_factory())
        .nest("/credential", credential_views_factory())
}
