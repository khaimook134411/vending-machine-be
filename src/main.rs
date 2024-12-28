pub mod config;
pub mod products;
mod init_database;
pub mod categories;

use axum::{http::Method, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};

// #[tokio::main]
async fn start_server() {
    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ]))
        .route("/categories", get(categories::handler::get_categories_router))
        .route("/category/create", post(categories::handler::create_category_router))
        .route("/product/create", post(products::handler::create_product_router))
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    init_database::init_database().await;
    start_server().await;
}
