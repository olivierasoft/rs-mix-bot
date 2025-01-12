use sea_orm::DatabaseConnection;
use serenity::{all::GatewayIntents, Client};
use std::error::Error;
use std::{env, sync::Arc};

use super::{
    enums::{DiscordEnv, Environment},
    event::DiscordInstance,
};

pub async fn retrieve_client(database: Arc<DatabaseConnection>) -> Result<Client, Box<dyn Error>> {
    let token = env::var(DiscordEnv::DiscordToken.as_str()).expect("Discord token not found");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let instance = DiscordInstance {
        db: Arc::clone(&database),
    };

    let client = Client::builder(token, intents)
        .event_handler(instance)
        .await
        .expect("Error while creating client");

    Ok(client)
}
