use std::path::Path;
use std::{fs::File, sync::Arc};

use sea_orm::{Database, DatabaseConnection};

pub async fn connect_to_sqlite() -> Arc<DatabaseConnection> {
    const SQLITE_PATH: &str = "mix.sqlite";

    if !Path::new(SQLITE_PATH).exists() {
        println!("Database file not exists, creating...");

        File::create(SQLITE_PATH).expect("Failed to create sqlite file!");
    };

    let database_url = format!("sqlite://{}", SQLITE_PATH);

    let database = Database::connect(database_url)
        .await
        .expect("Failed to create database connection");

    Arc::new(database)
}
