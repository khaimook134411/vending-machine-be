use crate::config::database::db_connect;
use crate::entities::products::{
    InsertProductRequest, Product, RemoveProductQuantityRequest, UpdateProductRequest,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use std::fmt::Debug;
use std::time::SystemTime;

pub async fn get_product(id: String) -> Result<Product, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT * FROM products WHERE id = $1";

            match client.query_one(query, &[&id]).await {
                Ok(row) => {
                    let product = Product {
                        id: row.get("id"),
                        title: row.get("title"),
                        category_id: row.get("category_id"),
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
                    };

                    Ok(product)
                }
                Err(e) => Err(format!("Product not found: {}", e)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

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
                            category_id: row.get("category_id"),
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

pub async fn update_product(req: UpdateProductRequest) -> Result<ObjectId, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "
                UPDATE products
                SET
                    title = COALESCE($2, title),
                    category_id = COALESCE($3, category_id),
                    description = COALESCE($4, description),
                    price = COALESCE($5, price),
                    quantity = COALESCE($6, quantity),
                    image_uri = COALESCE($7, image_uri),
                    deleted = COALESCE($8, deleted),
                    updated_at = NOW()
                WHERE id = $1;
            ";

            match client
                .execute(
                    query,
                    &[
                        &req.id.to_string(),
                        &req.title,
                        &req.category_id,
                        &req.description,
                        &req.price,
                        &req.quantity,
                        &req.image_uri,
                        &req.deleted,
                    ],
                )
                .await
            {
                Ok(rows_updated) => {
                    if rows_updated > 0 {
                        Ok(req.id.parse().unwrap())
                    } else {
                        Err("No product found with the given ID".to_string())
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn remove_product_quantity(
    req: RemoveProductQuantityRequest,
) -> Result<ObjectId, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "
                UPDATE products
                SET quantity = GREATEST(quantity - $2, 0), updated_at = NOW()
                WHERE id = $1
            ;";

            match client
                .execute(query, &[&req.id.to_string(), &req.quantity])
                .await
            {
                Ok(rows_affected) => {
                    if rows_affected == 1 {
                        Ok(req.id.parse().unwrap())
                    } else {
                        Err("No product found with the given ID.".to_string())
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

