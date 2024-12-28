use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: f32,
    pub image_uri: Option<String>,
    pub deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductBson {
    pub id: ObjectId,
    pub title: String,
    pub category_id: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: f32,
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
    pub deleted: bool,
}

#[derive(Serialize)]
pub struct InsertProductResponse {
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
            quantity: 0.0,
            image_uri: None,
            deleted: false,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }
}
