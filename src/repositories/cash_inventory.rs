use crate::config::database::db_connect;
use crate::entities::cash_inventory::CashInventory;

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
