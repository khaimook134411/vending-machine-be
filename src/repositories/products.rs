use std::fmt::Debug;
use crate::config::database::db_connect;
use crate::entities::products::{InsertProductRequest, Product};
use bson::oid::ObjectId;
use std::time::SystemTime;
use chrono::{DateTime, Utc};


pub async fn get_products() -> Result<Vec<Product>, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT * FROM products";

            match client.query(query, &[]).await {
                Ok(rows) => {
                    let products: Vec<Product> = rows
                        .into_iter()
                        .map(|row| Product {
                            id: row.get("id"),
                            title: row.get("title"),
                            category_id: "".to_string(),
                            description: row.get("description"),
                            price: row.get("price"),
                            quantity: row.get("quantity"),
                            image_uri: row.get("image_uri"),
                            deleted: row.get("deleted"),
                            created_at: {
                                let system_time: SystemTime = row.get("created_at");
                                let datetime: DateTime<Utc> = system_time.into(); // Convert to DateTime<Utc>
                                datetime.to_rfc3339() // Convert to ISO 8601 string
                            },
                            updated_at: {
                                let system_time: SystemTime = row.get("updated_at");
                                let datetime: DateTime<Utc> = system_time.into();
                                datetime.to_rfc3339()
                            },
                        })
                        .collect();

                    Ok(products)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_product(req: InsertProductRequest) -> Result<ObjectId, String> {
    match db_connect().await {
        Ok(client) => {
            let id = ObjectId::new();
            let query = "
                INSERT INTO products (id, title, category_id, description, price, quantity, image_uri, deleted)
                VALUES ($1, $2, $3, $4, $5, $6, $7, false);
            ";

            match client
                .execute(
                    query,
                    &[
                        &id.to_string(),
                        &req.title,
                        &req.category_id,
                        &req.description,
                        &req.price,
                        &req.quantity,
                        &req.image_uri,
                    ],
                )
                .await
            {
                Ok(..) => Ok(id),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
