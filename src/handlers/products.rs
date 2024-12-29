use crate::entities::products::{InsertProductRequest, ProductResponse, UpdateProductRequest};
use crate::repositories::products::{create_product, get_product, get_products, update_product};
use axum::http::StatusCode;
use axum::{
    extract::{Json, Path},
    response::{IntoResponse, Response},
};

pub async fn get_product_router(Path(id): Path<String>) -> Result<Response, StatusCode> {
    match get_product(id).await {
        Ok(product) => Ok(Json(product).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_products_router() -> Result<Response, StatusCode> {
    match get_products().await {
        Ok(products) => Ok(Json(products).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_product_router(
    Json(req): Json<InsertProductRequest>,
) -> Result<Response, StatusCode> {
    match create_product(InsertProductRequest {
        title: req.title.clone(),
        category_id: req.category_id.clone(),
        description: req.description.clone(),
        price: req.price.clone(),
        quantity: req.quantity.clone(),
        image_uri: req.image_uri.clone(),
    })
    .await
    {
        Ok(id) => Ok(Json(ProductResponse { id }).into_response().into_response()),
        Err(_err) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn update_product_router(
    Json(req): Json<UpdateProductRequest>,
) -> Result<Response, StatusCode> {
    match update_product(UpdateProductRequest {
        id: req.id.clone(),
        title: req.title.clone(),
        category_id: req.category_id.clone(),
        description: req.description.clone(),
        price: req.price.clone(),
        quantity: req.quantity.clone(),
        image_uri: req.image_uri.clone(),
        deleted: req.deleted.clone(),
    })
    .await
    {
        Ok(id) => Ok(Json(ProductResponse { id }).into_response().into_response()),
        Err(_err) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}
