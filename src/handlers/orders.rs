use crate::entities::orders::{CreateOrderRequest, CreateOrderResponse};
use crate::repositories::orders::create_order;
use axum::http::StatusCode;
use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};

pub async fn create_order_router(
    Json(req): Json<CreateOrderRequest>,
) -> Result<Response, StatusCode> {
    match create_order(CreateOrderRequest {
        product_id: req.product_id.clone(),
        quantity: req.quantity.clone(),
    })
    .await
    {
        Ok(id) => Ok(Json(CreateOrderResponse { id }).into_response()),
        Err(_err) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}
