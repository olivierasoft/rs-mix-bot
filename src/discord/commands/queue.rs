use super::super::event::DiscordInstance;
use crate::database::entities::prelude::User;
use crate::database::entities::{discord, user};
use crate::database::repository::guild_repository::GuildRepository;
use crate::database::repository::queue_repository::QueueRepository;
use crate::discord::commands::guild::verify_guild;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use serenity::all::{Channel, ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

async fn remove_message<'a>(ctx: &Context, msg: &'a Message) -> Result<(), Box<dyn Error>> {
    let ctx = Arc::new(ctx.clone());
    let msg = Arc::new(msg.clone());

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;

        if let Err(err) = msg.delete(&ctx.http).await {
            eprintln!("Failed to delete message: {:?}", err);
        }
    });

    Ok(())
}
async fn add_user_to_queue(
    db: &DatabaseConnection,
    interaction: &ComponentInteraction,
    ctx: &Context,
    queue_id: &String,
    user_id: &String,
) -> Result<(), Box<dyn Error>> {
    let queue_repository = QueueRepository::new(db);

    if let Err(_) = queue_repository.push_to_queue(queue_id, user_id).await {
        let interaction_response = CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content("You are already in the queue, wait for start...");

        interaction.create_response(ctx, CreateInteractionResponse::Message(interaction_response)).await?;

        return Ok(());
    }

    let count = queue_repository.count_queue(queue_id).await?;

    let message = interaction.message
        .reply(
            &ctx.http,
            &format!(
                "User <@{}> entered in queue, has {} users in queue",
                user_id, count
            ),
        )
        .await?;

    interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await?;

    remove_message(ctx, &message).await?;

    Ok(())
}

impl DiscordInstance {
    pub async fn queue_push(
        &self,
        interaction: &ComponentInteraction,
        ctx: &Context,
        user: user::Model,
    ) -> Result<(), Box<dyn Error>> {
        let guild_repository = GuildRepository::new(self.db.as_ref());

        let guild_id = interaction.guild_id.unwrap();

        let guild = guild_repository.find_guild_queue(&guild_id.to_string()).await?;

        if let Some(queue) = guild.queue {
            add_user_to_queue(&self.db.as_ref(), interaction, ctx, &guild_id.to_string(), &user.id.to_string()).await?;
        } else {
            guild_repository.create_guild_queue(&guild.guild.id).await?;
            add_user_to_queue(&self.db.as_ref(), interaction, ctx, &guild.guild.id, &user.id.to_string()).await?;
        }

        Ok(())
    }
    pub async fn join_queue(
        &self,
        interaction: &ComponentInteraction,
        ctx: &Context,
    ) -> Result<(), Box<dyn Error>> {
        match interaction.message.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                let guild_id = interaction.guild_id.expect("GuildId not found");

                match User::find_by_id(interaction.message.author.id.get().to_string())
                    .one(self.db.as_ref())
                    .await?
                {
                    Some(user) => {
                        self.queue_push(&interaction, ctx, user)
                            .await?;
                    }
                    None => {
                        let txn = self.db.as_ref().begin().await?;

                        let id = Set(interaction.message.author.id.get().to_string());

                        discord::ActiveModel {
                            id: id.clone(),
                            name: Set(interaction.message.author.name.to_owned()),
                            global_name: Set(interaction.message.author.global_name.to_owned()),
                            email: Set(interaction.message.author.email.to_owned()),
                            discriminator: Set(interaction.message.author.discriminator.map(|n| n.to_string())),
                        }
                            .insert(&txn)
                            .await?;

                        let user = user::ActiveModel {
                            id: id.clone(),
                            name: Set(interaction.message.author.name.to_owned()),
                        }
                            .insert(&txn)
                            .await?;

                        if let Err(_) = txn.commit().await {
                            interaction.message.reply(&ctx.http, "User creation failed, try again...")
                                .await?;
                        }

                        verify_guild(self.db.as_ref(), &ctx, guild_id, interaction.message.author.clone()).await?;

                        self.queue_push(&interaction, ctx, user)
                            .await?;
                    }
                }
            }
            _ => {
                interaction.message.reply(&ctx.http, "This command can only be used in a server")
                    .await?;
            }
        }

        Ok(())
    }
}
