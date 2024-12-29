use crate::entities::cash_inventory::UpdateCashInventoryRequest;
use crate::repositories::cash_inventory::{get_cash_inventory, update_cash_inventory};
use axum::http::StatusCode;
use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};

pub async fn get_cash_inventory_router() -> Result<Response, StatusCode> {
    match get_cash_inventory().await {
        Ok(cash_inventory) => Ok(Json(cash_inventory).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_cash_inventory_router(
    Json(req): Json<UpdateCashInventoryRequest>,
) -> Result<Response, StatusCode> {
    match update_cash_inventory(UpdateCashInventoryRequest {
        coin_1: req.coin_1.clone(),
        coin_5: req.coin_5.clone(),
        coin_10: req.coin_10.clone(),
        bank_20: req.bank_20.clone(),
        bank_50: req.bank_50.clone(),
        bank_100: req.bank_100.clone(),
        bank_500: req.bank_500.clone(),
        bank_1000: req.bank_1000.clone(),
    })
    .await
    {
        Ok(cash_inventory) => Ok(Json(cash_inventory).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
