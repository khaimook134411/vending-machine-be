use crate::entities::orders::{
    CancelOrderRequest, CancelOrderResponse, CompleteOrderRequest, CompleteOrderResponse,
    CreateOrderRequest, CreateOrderResponse,
};
use crate::repositories::orders::{
    cancel_order, complete_order, create_order, get_order, get_orders,
};
use axum::http::StatusCode;
use axum::{
    extract::{Json, Path},
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub async fn get_order_router(Path(id): Path<String>) -> Result<Response, StatusCode> {
    match get_order(id).await {
        Ok(order) => Ok(Json(order).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_orders_router() -> Result<Response, StatusCode> {
    match get_orders().await {
        Ok(orders) => Ok(Json(orders).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

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

pub async fn cancel_order_router(
    Json(req): Json<CancelOrderRequest>,
) -> Result<Response, StatusCode> {
    match cancel_order(CancelOrderRequest {
        id: req.id.clone(),
        refund: req.refund.clone(),
    })
    .await
    {
        Ok(id) => Ok(Json(CancelOrderResponse {
            id,
            refund: req.refund.clone(),
        })
        .into_response()),
        Err(_err) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

#[derive(Serialize)]
struct ErrResponse {
    message: String,
}

pub async fn complete_order_router(
    Json(req): Json<CompleteOrderRequest>,
) -> Result<Response, StatusCode> {
    match complete_order(CompleteOrderRequest {
        id: req.id.clone(),
        receive: req.receive.clone(),
    })
    .await
    {
        Ok(change) => {
            let order = get_order(req.id.clone()).await.unwrap();

            Ok(Json(CompleteOrderResponse {
                id: req.id.clone(),
                total: order.total.clone(),
                receive: req.receive,
                change,
            })
            .into_response())
        }
        Err(e) => Ok(Json(ErrResponse {
            message: e.to_string(),
        })
        .into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
