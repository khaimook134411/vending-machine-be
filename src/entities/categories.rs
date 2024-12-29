use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCategoryRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: ObjectId,
}

impl Category {
    pub fn new() -> Self {
        Category {
            id: "".to_string(),
            title: "".to_string(),
            description: None,
            deleted: false,
        }
    }
}
