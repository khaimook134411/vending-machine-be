use crate::config::database::db_connect;

pub async fn init_database() {
    let client = db_connect();

    match client.await {
        Ok(mut client) => {
            // Create the `categories` table
            client.execute("
            CREATE TABLE IF NOT EXISTS categories (
                id VARCHAR(255) PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        ", &[]).await.expect("cannot create categories table");

            // Create the `products` table
            client.execute("
              CREATE TABLE IF NOT EXISTS products (
                id VARCHAR(255) PRIMARY KEY,
                category_id VARCHAR(255),
                title VARCHAR(255),
                description VARCHAR(255),
                price DOUBLE PRECISION,
                quantity INTEGER,
                image_url VARCHAR(255),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );", &[]).await.expect("cannot create postgres connection");
        },
        Err(e) => panic!("{}", e),
    }
}