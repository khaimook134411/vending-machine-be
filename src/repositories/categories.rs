use crate::config::database::db_connect;
use bson::oid::ObjectId;
use crate::entities::categories::{Category, InsertCategoryRequest, UpdateCategoryRequest};

pub async fn get_categories() -> Result<Vec<Category>, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT id, title, description FROM categories";

            match client.query(query, &[]).await {
                Ok(rows) => {
                    let categories: Vec<Category> = rows
                        .into_iter()
                        .map(|row| Category {
                            id: row.get("id"),
                            title: row.get("title"),
                            description: row.get("description"),
                            deleted: false,
                        })
                        .collect();

                    Ok(categories)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
pub async fn create_category(req: InsertCategoryRequest) -> Result<ObjectId, String> {
    match db_connect().await {
        Ok(client) => {
            let id = ObjectId::new();
            let query = "
                INSERT INTO categories (id, title, description, deleted)
                VALUES ($1, $2, $3, false);
            ";

            match client
                .execute(query, &[&id.to_string(), &req.title, &req.description])
                .await
            {
                Ok(..) => Ok(id),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn update_category(req: UpdateCategoryRequest) -> Result<ObjectId, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "
                UPDATE categories
                SET
                    title = COALESCE($2, title),
                    description = COALESCE($3, description),
                    deleted = COALESCE($4, deleted)
                WHERE id = $1;
            ";

            match client
                .execute(query, &[&req.id.to_string(), &req.title, &req.description, &req.deleted])
                .await
            {
                Ok(rows_updated) => {
                    if rows_updated > 0 {
                        Ok(req.id.parse().unwrap())
                    } else {
                        Err("No category found with the given ID".to_string())
                    }
                }
                Err(e) => Err(e.to_string()),
            }

        }
        Err(e) => Err(e.to_string()),
    }
}
