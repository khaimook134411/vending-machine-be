use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub description: Option<String>,
    pub price: f64,
    pub quantity: i32,
    pub image_uri: Option<String>,
    pub deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertProductRequest {
    pub title: String,
    pub category_id: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub quantity: i32,
    pub image_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub id: String,
    pub title: Option<String>,
    pub category_id: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub image_uri: Option<String>,
    pub deleted: Option<bool>,
}

pub struct RemoveProductQuantityRequest {
    pub id: String,
    pub quantity: i32,
}


#[derive(Serialize)]
pub struct ProductResponse {
    pub id: ObjectId,
}

impl Product {
    pub fn new() -> Self {
        Product {
            id: "".to_string(),
            title: "".to_string(),
            category_id: "".to_string(),
            description: None,
            price: 0.0,
            quantity: 0,
            image_uri: None,
            deleted: false,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
