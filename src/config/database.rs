use tokio_postgres::{NoTls, Error, Client};

pub async fn db_connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=vending_postgres user=postgres password=password dbname=vending_machine port=5432",
        NoTls,
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
