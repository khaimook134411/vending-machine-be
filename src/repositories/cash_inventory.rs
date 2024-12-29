use crate::config::database::db_connect;
use crate::entities::cash_inventory::{CashInventory, UpdateCashInventoryRequest};

pub async fn get_cash_inventory() -> Result<CashInventory, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "SELECT * FROM cash_inventory WHERE id = 0";

            match client.query(query, &[]).await {
                Ok(row) => {
                    let cash_inventory = CashInventory {
                        id: 0,
                        coin_1: row[0].get("coin_1"),
                        coin_5: row[0].get("coin_5"),
                        coin_10: row[0].get("coin_10"),
                        bank_20: row[0].get("bank_20"),
                        bank_50: row[0].get("bank_50"),
                        bank_100: row[0].get("bank_100"),
                        bank_500: row[0].get("bank_500"),
                        bank_1000: row[0].get("bank_1000"),
                    };
                    Ok(cash_inventory)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn update_cash_inventory(
    req: UpdateCashInventoryRequest,
) -> Result<CashInventory, String> {
    match db_connect().await {
        Ok(client) => {
            let query = "
                UPDATE cash_inventory
                SET
                    coin_1 = COALESCE($1, coin_1),
                    coin_5 = COALESCE($2, coin_5),
                    coin_10 = COALESCE($3, coin_10),
                    bank_20 = COALESCE($4, bank_20),
                    bank_50 = COALESCE($5, bank_50),
                    bank_100 = COALESCE($6, bank_100),
                    bank_500 = COALESCE($7, bank_500),
                    bank_1000 = COALESCE($8, bank_1000)
                WHERE id = 0;
            ";

            match client
                .execute(
                    query,
                    &[
                        &req.coin_1,
                        &req.coin_5,
                        &req.coin_10,
                        &req.bank_20,
                        &req.bank_50,
                        &req.bank_100,
                        &req.bank_500,
                        &req.bank_1000,
                    ],
                )
                .await
            {
                Ok(rows_updated) => {
                    if rows_updated > 0 {
                        let select_query = "SELECT * FROM cash_inventory WHERE id = 0";
                        match client.query_one(select_query, &[]).await {
                            Ok(row) => {
                                let cash_inventory = CashInventory {
                                    id: row.get("id"),
                                    coin_1: row.get("coin_1"),
                                    coin_5: row.get("coin_5"),
                                    coin_10: row.get("coin_10"),
                                    bank_20: row.get("bank_20"),
                                    bank_50: row.get("bank_50"),
                                    bank_100: row.get("bank_100"),
                                    bank_500: row.get("bank_500"),
                                    bank_1000: row.get("bank_1000"),
                                };
                                Ok(cash_inventory)
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    } else {
                        Err("No rows were updated".to_string())
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
