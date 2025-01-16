use std::sync::{Arc, Mutex};

use crate::database::entities::user;
use sea_orm::DatabaseConnection;
use serenity::all::{
    CreateInteractionResponse, Interaction, InteractionType,
};
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
            if let Err(why) = self.join_queue(&ctx, &msg).await {
                println!("Error sending message: {:?}", why);
            }
        };
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::Component => {
                let component_interaction = interaction
                    .as_message_component()
                    .expect("Failed to get component interaction");

                self.join_queue(&ctx, component_interaction.message.as_ref())
                    .await
                    .expect("Failed to join queue");

                component_interaction
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Acknowledge,
                    )
                    .await
                    .expect("Failed to acknowledge interaction");
            }
            _ => {}
        }
    }
}
