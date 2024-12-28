use postgres::{Client, Error, NoTls};

pub fn db_connect() -> Result<Client, Error> {
    let mut client = Client::connect(
        "postgres://postgres:password@localhost:5433/vending_machine",
        NoTls,
    );

    client
}
