pub mod config;
mod init_database;
mod entities;
mod repositories;
mod handlers;

use axum::{http::Method, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::cash_inventory::{get_cash_inventory_router, update_cash_inventory_router};
use crate::handlers::categories::{create_category_router, get_categories_router, update_category_router};
use crate::handlers::orders::{cancel_order_router, create_order_router, get_orders_router};
use crate::handlers::products::{create_product_router, get_product_router, get_products_router, update_product_router};
use crate::repositories::orders::get_orders;

async fn start_server() {
    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ]))
        .route("/categories", get(get_categories_router))
        .route("/category/create", post(create_category_router))
        .route("/category/update", post(update_category_router))
        .route("/products", get(get_products_router))
        .route("/products/:id", get(get_product_router))
        .route("/product/create", post(create_product_router))
        .route("/product/update", post(update_product_router))
        .route("/cash_inventory", get(get_cash_inventory_router))
        .route("/cash_inventory/update", post(update_cash_inventory_router))
        .route("/orders", get(get_orders_router))
        .route("/order/create", post(create_order_router))
        .route("/order/cancel", post(cancel_order_router))
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
