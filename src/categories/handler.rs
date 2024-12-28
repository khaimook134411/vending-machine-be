use axum::{
    extract::{Json},
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;
use crate::categories::entity::{Category, InsertCategoryRequest, InsertCategoryResponse};
use crate::categories::repository::{create_category, get_categories};

pub async fn get_categories_router() -> Result<Response, StatusCode> {
    match get_categories().await {
        Ok(categories) => {
            Ok(Json(categories).into_response())
        }
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_category_router(Json(req): Json<InsertCategoryRequest>
) -> Result<Response, StatusCode> {

    match create_category(InsertCategoryRequest {
        title: req.title.clone(),
        description: req.description.clone(),
    }).await {
        Ok(id) => {
            Ok(Json(InsertCategoryResponse {
                id,
            }).into_response().into_response())
        }
        Err(_err) => {
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        },
    }
}
