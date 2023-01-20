use axum::Router;
use http::Method;
use tower_http::cors::{Any, CorsLayer};
use views::views_factory;

mod error;
mod request;
mod views;

#[tokio::main]
async fn main() {
    // initialize logger
    env_logger::init();

    // CORS middleware layer.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // build application with shared database and CORS-layer
    let app = Router::new().nest("/api/v1", views_factory()).layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
