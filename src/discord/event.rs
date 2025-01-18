use std::sync::{Arc, Mutex};

use crate::database::entities::user;
use crate::discord::enums::{Environment, MixEvents};
use sea_orm::DatabaseConnection;
use serenity::all::{CreateInteractionResponse, Interaction, InteractionType};
use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

pub struct DiscordInstance {
    pub db: Arc<DatabaseConnection>,
    pub queue: Mutex<Vec<user::Model>>,
}
#[async_trait]
impl EventHandler for DiscordInstance {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!configure" {
            if let Err(why) = self.configure(&ctx, &msg).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!queue" {
            if let Err(why) = self.join_queue(&ctx, &msg, msg.guild_id).await {
                println!("Error sending message: {:?}", why);
            }
        };
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::Component => {
                let interaction = interaction
                    .as_message_component()
                    .expect("Failed to get component interaction");

                if interaction.data.custom_id == MixEvents::JoinQueue.as_str() {
                    self.join_queue(&ctx, interaction.message.as_ref(), interaction.guild_id)
                        .await
                        .expect("Failed to join queue");
                }

                if interaction.data.custom_id == MixEvents::LeftQueue.as_str() {}
                interaction
                    .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
                    .await
                    .expect("Failed to acknowledge interaction");
            }
            _ => {}
        }
    }
}
