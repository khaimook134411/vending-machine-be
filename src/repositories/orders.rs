use crate::config::database::db_connect;
use crate::entities::cash_inventory::CashInventory;
use crate::entities::orders::{
    CancelOrderRequest, CompleteOrderRequest, CreateOrderRequest, Money, Order,
};
use crate::entities::products::RemoveProductQuantityRequest;
use crate::repositories::cash_inventory::get_cash_inventory;
use crate::repositories::products::{get_product, remove_product_quantity, update_product};
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

pub async fn complete_order(req: CompleteOrderRequest) -> Result<Money, String> {
    match db_connect().await {
        Ok(client) => {
            let order = get_order(req.id.clone()).await.unwrap();
            let change = order.total.clone() - sum_money(req.receive.clone());
            let breakdown_change = breakdown_change_into_money(change.clone());
            let cash_inventory = get_cash_inventory().await;
            let has_enough_change =
                check_has_enough_change(change as i32, cash_inventory.unwrap());
            let status = if has_enough_change {
                "COMPLETED".to_string()
            } else {
                "CANCELLED".to_string()
            };

            let query = "
                UPDATE orders
                SET
                    status = $2
                WHERE id = $1;
            ";

            if has_enough_change {
                remove_product_quantity(RemoveProductQuantityRequest {
                    id: order.product_id.clone(),
                    quantity: order.quantity.clone(),
                })
                .await
                .expect("TODO: panic message");
            } else {
                return Err("Order not completed".to_string());
            };

            match client.execute(query, &[&req.id, &status]).await {
                Ok(rows_updated) => Ok(breakdown_change),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

// fn check_has_enough_change(change: Money, cash_inventory: CashInventory) -> bool {
//     // TODO: improve function
//     change.coin_1 <= cash_inventory.coin_1
//         && change.coin_5 <= cash_inventory.coin_5
//         && change.coin_10 <= cash_inventory.coin_1
//         && change.bank_20 <= cash_inventory.bank_20
//         && change.bank_50 <= cash_inventory.bank_50
//         && change.bank_100 <= cash_inventory.bank_100
//         && change.bank_500 <= cash_inventory.bank_500
//         && change.bank_1000 <= cash_inventory.bank_1000
// }
fn check_has_enough_change(change: i32, cash_inventory: CashInventory) -> bool {
    let denominations = [
        (1000, cash_inventory.bank_1000),
        (500, cash_inventory.bank_500),
        (100, cash_inventory.bank_100),
        (50, cash_inventory.bank_50),
        (20, cash_inventory.bank_20),
        (10, cash_inventory.coin_10),
        (5, cash_inventory.coin_5),
        (1, cash_inventory.coin_1),
    ];

    let mut remaining_change = change;

    for (denom, available) in denominations.iter() {
        // Dereference `available` to get the value
        let available_value = *available; // Dereferencing here
        let count = remaining_change / (denom.min(&available_value));
        remaining_change -= count * denom;

        if remaining_change == 0 {
            return true;  // Successfully made the change
        }
    }

    remaining_change == 0  // If all the required change has been covered
}


fn sum_money(money: Money) -> f64 {
    (money.coin_1 as f64 * 0.01)
        + (money.coin_5 as f64 * 0.05)
        + (money.coin_10 as f64 * 0.10)
        + (money.bank_20 as f64 * 20.0)
        + (money.bank_50 as f64 * 50.0)
        + (money.bank_100 as f64 * 100.0)
        + (money.bank_500 as f64 * 500.0)
        + (money.bank_1000 as f64 * 1000.0)
}

pub fn breakdown_change_into_money(mut change: f64) -> Money {
    const COIN_1: f64 = 0.01;
    const COIN_5: f64 = 0.05;
    const COIN_10: f64 = 0.10;
    const BANK_20: f64 = 20.0;
    const BANK_50: f64 = 50.0;
    const BANK_100: f64 = 100.0;
    const BANK_500: f64 = 500.0;
    const BANK_1000: f64 = 1000.0;

    // Calculate how many units fit for each denomination
    let bank_1000 = (change / BANK_1000).floor() as i32;
    change -= bank_1000 as f64 * BANK_1000;

    let bank_500 = (change / BANK_500).floor() as i32;
    change -= bank_500 as f64 * BANK_500;

    let bank_100 = (change / BANK_100).floor() as i32;
    change -= bank_100 as f64 * BANK_100;

    let bank_50 = (change / BANK_50).floor() as i32;
    change -= bank_50 as f64 * BANK_50;

    let bank_20 = (change / BANK_20).floor() as i32;
    change -= bank_20 as f64 * BANK_20;

    let coin_10 = (change / COIN_10).floor() as i32;
    change -= coin_10 as f64 * COIN_10;

    let coin_5 = (change / COIN_5).floor() as i32;
    change -= coin_5 as f64 * COIN_5;

    let coin_1 = (change / COIN_1).round() as i32;

    Money {
        coin_1,
        coin_5,
        coin_10,
        bank_20,
        bank_50,
        bank_100,
        bank_500,
        bank_1000,
    }
}
