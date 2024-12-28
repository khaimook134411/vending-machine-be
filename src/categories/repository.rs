use crate::categories::entity::{Category, InsertCategoryRequest};
use crate::config::database::db_connect;
use bson::oid::ObjectId;

pub async fn get_categories() -> Result<Vec<Category>, String> {
    match db_connect().await {
        Ok(mut client) => {
            let query = "SELECT id, title, description FROM categories";

            match client.query(query, &[]).await {
                Ok(rows) => {
                    let categories: Vec<Category> = rows
                        .into_iter()
                        .map(|row| Category {
                            id: row.get("id"),
                            title: row.get("title"),
                            description: row.get("description"),
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
        Ok(mut client) => {
            let id = ObjectId::new();
            let query = "
                INSERT INTO categories (id, title, description)
                VALUES ($1, $2, $3);
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
