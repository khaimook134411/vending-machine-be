use axum::{
    extract::{Json},
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;
use crate::repositories::cash_inventory::get_cash_inventory;

pub async fn get_cash_inventory_router() -> Result<Response, StatusCode> {
    match get_cash_inventory().await {
        Ok(cash_inventory) => {
            Ok(Json(cash_inventory).into_response())
        }
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}