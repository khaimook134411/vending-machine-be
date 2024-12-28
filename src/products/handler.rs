use axum::{
    extract::{Json},
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;
use crate::products::entity::{InsertProductRequest, InsertProductResponse};
use crate::products::repository::create_product;

pub async fn create_product_router(Json(req): Json<InsertProductRequest>
) -> Result<Response, StatusCode> {

    match create_product(InsertProductRequest {
        title: req.title.clone(),
        category_id: req.category_id.clone(),
        description: req.description.clone(),
        price: req.price.clone(),
        quantity: req.quantity.clone(),
        image_uri: req.image_uri.clone(),
    }).await {
        Ok(id) => {
            Ok(Json(InsertProductResponse {
                id,
            }).into_response().into_response())
        }
        Err(_err) => {
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        },
    }
}
