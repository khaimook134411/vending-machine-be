use crate::config::database::db_connect;

pub async fn init_database() {
    let client =  match db_connect().await {
        Ok(mut client) => client,
        Err(e) => panic!("Error: Database connection fialed:  {:?}",e)
    };

    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;
}