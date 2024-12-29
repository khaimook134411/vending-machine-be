use crate::config::database::db_connect;

pub async fn init_database() {
    let client = db_connect();

    match client.await {
        Ok(client) => {
            // Create the `categories` table
            client
                .execute(
                    "
            CREATE TABLE IF NOT EXISTS categories (
                id VARCHAR(255) PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                deleted BOOLEAN,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
                    &[],
                )
                .await
                .expect("cannot create categories table");

            // Create the `products` table
            client
                .execute(
                    "
              CREATE TABLE IF NOT EXISTS products (
                id VARCHAR(255) PRIMARY KEY,
                category_id VARCHAR(255),
                title VARCHAR(255),
                description VARCHAR(255),
                price DOUBLE PRECISION,
                quantity INTEGER,
                image_uri VARCHAR(255),
                deleted BOOLEAN,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
                    &[],
                )
                .await
                .expect("cannot create postgres connection");

            // Create the `cash_inventory` table
            client
                .execute(
                    "
              CREATE TABLE IF NOT EXISTS cash_inventory (
                id INTEGER PRIMARY KEY,
                coin_1 INTEGER,
                coin_5 INTEGER,
                coin_10 INTEGER,
                bank_20 INTEGER,
                bank_50 INTEGER,
                bank_100 INTEGER,
                bank_500 INTEGER,
                bank_1000 INTEGER
            );",
                    &[],
                )
                .await
                .expect("cannot create postgres connection");
            client
                .execute(
                    "
              INSERT INTO cash_inventory (id, coin_1, coin_5, coin_10, bank_20, bank_50, bank_100, bank_500, bank_1000)
              VALUES (0, 0, 0, 0, 0, 0, 0, 0, 0)
              ON CONFLICT (id) DO NOTHING;
        ", &[]
                )
                .await
                .expect("cannot create postgres connection");

            // Create the `orders` table
            client
                .execute(
                    "
            CREATE TABLE IF NOT EXISTS orders (
                id VARCHAR(255) PRIMARY KEY,
                product_id VARCHAR(255) NOT NULL,
                quantity INTEGER NOT NULL,
                total DOUBLE PRECISION NOT NULL,
                status VARCHAR(255) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
                    &[],
                )
                .await
                .expect("cannot create orders table");
        }
        Err(e) => panic!("{}", e),
    }
}
