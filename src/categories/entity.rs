use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryBson {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertCategoryRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct InsertCategoryResponse {
    pub id: ObjectId,
}

impl Category {
    pub fn new() -> Self {
        Category {
            id: "".to_string(),
            title: "".to_string(),
            description: None,
        }
    }
}
