use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: f32,
    pub image_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductBson {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: f32,
    pub image_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertProductRequest {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: f32,
    pub image_uri: Option<String>,
}

impl Product {
    pub fn new() -> Self {
        Product {
            id: "".to_string(),
            title: "".to_string(),
            description: None,
            price: 0.0,
            quantity: 0.0,
            image_uri: None,
        }
    }
}
