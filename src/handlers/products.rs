use axum::{
    extract::{Json},
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;
use crate::entities::products::{InsertProductRequest, InsertProductResponse};
use crate::repositories::products::{create_product, get_products};

pub async fn get_products_router() -> Result<Response, StatusCode> {
    match get_products().await {
        Ok(products) => {
            Ok(Json(products).into_response())
        }
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

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
