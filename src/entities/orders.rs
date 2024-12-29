use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub quantity: i32,
    pub total: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct CreateOrderResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    pub id: String,
    pub refund: Money
}

#[derive(Serialize)]
pub struct CancelOrderResponse {
    pub id: String,
    pub refund: Money
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteOrderRequest {
    pub id: String,
    pub receive: Money
}

#[derive(Serialize)]
pub struct CompleteOrderResponse {
    pub id: String,
    pub total: f64,
    pub receive: Money,
    pub change: Money
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    pub coin_1: i32,
    pub coin_5: i32,
    pub coin_10: i32,
    pub bank_20: i32,
    pub bank_50: i32,
    pub bank_100: i32,
    pub bank_500: i32,
    pub bank_1000: i32,
}

impl Order {
    pub fn new() -> Self {
        Order {
            id: "".to_string(),
            product_id: "".to_string(),
            quantity: 0,
            total: 0.0,
            status: "".to_string(),
        }
    }
}
