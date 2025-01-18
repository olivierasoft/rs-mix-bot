use super::{
    enums::{DiscordEnv, Environment},
    event::DiscordInstance,
};
use sea_orm::DatabaseConnection;
use serenity::{all::GatewayIntents, Client};
use std::error::Error;
use std::sync::Mutex;
use std::{env, sync::Arc};

pub async fn retrieve_client(database: Arc<DatabaseConnection>) -> Result<Client, Box<dyn Error>> {
    let token = env::var(DiscordEnv::DiscordToken.as_str()).expect("Discord token not found");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS;

    let instance = DiscordInstance {
        db: Arc::clone(&database),
        queue: Mutex::new(Vec::new()),
    };

    let client = Client::builder(token, intents)
        .event_handler(instance)
        .await
        .expect("Error while creating client");

    Ok(client)
}
