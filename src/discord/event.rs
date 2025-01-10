use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

pub struct DiscordInstance {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl EventHandler for DiscordInstance {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(error) = msg.delete(&ctx.http).await {
                println!("Error on delete message: {}", error);
            }
        }
    }
}
