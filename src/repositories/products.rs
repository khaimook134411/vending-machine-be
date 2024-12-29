use bson::oid::ObjectId;
use crate::config::database::db_connect;
use crate::entities::products::InsertProductRequest;

pub async fn create_product(req: InsertProductRequest) -> Result<ObjectId, String>{
    match db_connect().await {
        Ok(client) => {
            let id = ObjectId::new();
            let query = "
                INSERT INTO products (id, title, description, price, quantity, image_url)
                VALUES ($1, $2, $3, $4, $5, $6);
            ";

            match client.execute(
                query,
                &[
                    &id.to_string(),
                    &req.title,
                    &req.description,
                    &req.price,
                    &req.quantity,
                    &req.image_uri,
                ],
            ).await {
                Ok(..) => {
                    Ok(id)
                },
                Err(e) =>  {
                    Err(e.to_string())
                }
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}