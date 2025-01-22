extern crate dotenv;
mod database;
mod discord;

use database::connection;

use discord::client;

use dotenv::dotenv;

use database::repository::queue_repository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = connection::connect_to_sqlite().await;

    queue_repository::QueueRepository::new(database.as_ref())
        .purge_queue_users()
        .await
        .expect("Cannot purge queue users");

    let mut client = client::retrieve_client(database)
        .await
        .expect("Error while creating client");

    if let Err(err) = &client.start().await {
        println!("{err:?}");
    };
}
