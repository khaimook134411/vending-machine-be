use axum::{
    extract::{Json},
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;
use crate::entities::categories::{CategoryResponse, InsertCategoryRequest, UpdateCategoryRequest};
use crate::repositories::categories::{create_category, get_categories, update_category};

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
            Ok(Json(CategoryResponse {
                id,
            }).into_response())
        }
        Err(_err) => {
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        },
    }
}

pub async  fn update_category_router(Json(req): Json<UpdateCategoryRequest>) -> Result<Response, StatusCode>{
    match update_category(UpdateCategoryRequest {
        id: req.id.clone(),
        title: req.title.clone(),
        description: req.description.clone(),
        deleted: req.deleted.clone(),
    }).await{
        Ok(id) => {
            Ok(Json(CategoryResponse {
                id,
            }).into_response())
        }
        Err(_err) => {
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
