use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CashInventory {
    pub id: i32,
    pub coin_1: i32,
    pub coin_5: i32,
    pub coin_10: i32,
    pub bank_20: i32,
    pub bank_50: i32,
    pub bank_100: i32,
    pub bank_500: i32,
    pub bank_1000: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCashInventoryRequest {
    pub coin_1: Option<i32>,
    pub coin_5: Option<i32>,
    pub coin_10: Option<i32>,
    pub bank_20: Option<i32>,
    pub bank_50: Option<i32>,
    pub bank_100: Option<i32>,
    pub bank_500: Option<i32>,
    pub bank_1000: Option<i32>,
}

#[derive(Serialize)]
pub struct CashInventoryResponse {
    pub coin_1: i32,
    pub coin_5: i32,
    pub coin_10: i32,
    pub bank_20: i32,
    pub bank_50: i32,
    pub bank_100: i32,
    pub bank_500: i32,
    pub bank_1000: i32,
}

impl CashInventory {
    pub fn new() -> Self {
        CashInventory {
            id: 0,
            coin_1: 0,
            coin_5: 0,
            coin_10: 0,
            bank_20: 0,
            bank_50: 0,
            bank_100: 0,
            bank_500: 0,
            bank_1000: 0,
        }
    }
}
