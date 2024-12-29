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
pub struct InsertOrderRequest {
    pub product_id: String,
    pub quantity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: String,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: ObjectId,
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
