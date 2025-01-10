mod database;
mod discord;

use std::sync::Arc;

use database::connection;
use discord::client;

extern crate dotenv;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = connection::connect_to_sqlite().await;

    let mut client = client::retrieve_client(Arc::new(database))
        .await
        .expect("Error while creating client");

    if let Err(err) = &client.start().await {
        println!("{err:?}");
    };
}
