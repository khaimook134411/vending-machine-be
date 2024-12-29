use crate::config::database::db_connect;
use crate::entities::orders::{CancelOrderRequest, CreateOrderRequest, Order};
use crate::repositories::products::get_product;
use bson::oid::ObjectId;
use chrono::Utc;

pub async fn get_order(id: String) -> Result<Order, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT * FROM orders WHERE id = $1";

            match client.query_one(query, &[&id]).await {
                Ok(row) => {
                    let order = Order {
                        id,
                        product_id: row.get("product_id"),
                        quantity: row.get("quantity"),
                        total: row.get("total"),
                        status: row.get("status"),
                    };

                    Ok(order)
                }
                Err(e) => Err(format!("Order not found: {}", e)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
pub async fn get_orders() -> Result<Vec<Order>, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT * FROM orders";

            match client.query(query, &[]).await {
                Ok(rows) => {
                    let orders: Vec<Order> = rows
                        .into_iter()
                        .map(|row| Order {
                            id: row.get("id"),
                            product_id: row.get("product_id"),
                            quantity: row.get("quantity"),
                            total: row.get("total"),
                            status: row.get("status"),
                        })
                        .collect();

                    Ok(orders)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
pub async fn create_order(req: CreateOrderRequest) -> Result<String, String> {
    match db_connect().await {
        Ok(client) => {
            let order_id = generate_order_id();
            let product_result = get_product(req.product_id.clone()).await;
            let product_price = match product_result {
                Ok(product) => product.price,
                Err(e) => return Err(e),
            };
            let total = product_price * req.quantity.clone() as f64;

            let query = "
                INSERT INTO orders (id, product_id, quantity, total, status)
                VALUES ($1, $2, $3, $4, 'CREATED');
            ";

            match client
                .execute(query, &[&order_id, &req.product_id, &req.quantity, &total])
                .await
            {
                Ok(res) => Ok(order_id),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn generate_order_id() -> String {
    let date_prefix = Utc::now().format("%Y%m%d").to_string();
    let object_id = ObjectId::new().to_hex();
    format!("{}{}", date_prefix, object_id)
}

pub async fn cancel_order(req: CancelOrderRequest) -> Result<String, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "
                UPDATE orders
                SET
                    status = 'CANCELLED'
                WHERE id = $1;
            ";

            match client.execute(query, &[&req.id]).await {
                Ok(rows_updated) => Ok(req.id),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
