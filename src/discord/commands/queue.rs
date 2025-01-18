use super::super::event::DiscordInstance;
use crate::database::entities::prelude::User;
use crate::database::entities::{discord, user};
use crate::database::repository::guild_repository::GuildRepository;
use crate::discord::commands::guild::verify_guild;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TransactionTrait};
use serenity::all::{Channel, Context, GuildId, Message};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

impl DiscordInstance {
    pub async fn queue_push(
        &self,
        ctx: &Context,
        msg: &Message,
        guild_id: &String,
        user: user::Model,
    ) -> Result<(), Box<dyn Error>> {
        let guild_repository = GuildRepository::new(self.db.as_ref());


        let guild = guild_repository
            .find_guild_queue(guild_id)
            .await?;

        if let Some(queue) = guild.queue {
            // TODO
        } else {
            guild_repository.create_guild_queue(&guild.guild.id).await?;
        }

        let message = msg
            .reply(
                &ctx.http,
                format!("You placed in queue, has {} users in queue.", 0),
            )
            .await?;

        let ctx = Arc::new(ctx.clone());
        let message = Arc::new(message);

        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;

            if let Err(err) = message.delete(&ctx.http).await {
                eprintln!("Failed to delete message: {:?}", err);
            }
        });

        Ok(())
    }
    pub async fn join_queue(
        &self,
        ctx: &Context,
        msg: &Message,
        guild_id: Option<GuildId>,
    ) -> Result<(), Box<dyn Error>> {
        match msg.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                let guild_id = guild_id.expect("GuildId not found");

                match User::find_by_id(msg.author.id.get().to_string())
                    .one(self.db.as_ref())
                    .await?
                {
                    Some(user) => {
                        self.queue_push(ctx, msg, &guild_id.to_string(), user).await?;
                    }
                    None => {
                        let txn = self.db.as_ref().begin().await?;

                        let id = Set(msg.author.id.get().to_string());

                        discord::ActiveModel {
                            id: id.clone(),
                            name: Set(msg.author.name.to_owned()),
                            global_name: Set(msg.author.global_name.to_owned()),
                            email: Set(msg.author.email.to_owned()),
                            discriminator: Set(msg.author.discriminator.map(|n| n.to_string())),
                        }
                            .insert(&txn)
                            .await?;

                        let user = user::ActiveModel {
                            id: id.clone(),
                            name: Set(msg.author.name.to_owned()),
                        }
                            .insert(&txn)
                            .await?;

                        if let Err(_) = txn.commit().await {
                            msg.reply(&ctx.http, "User creation failed, try again...")
                                .await?;
                        }

                        verify_guild(self.db.as_ref(), &ctx, guild_id, msg.author.clone())
                            .await?;

                        self.queue_push(ctx, msg, &guild_id.to_string(), user).await?;
                    }
                }
            }
            _ => {
                msg.reply(&ctx.http, "This command can only be used in a server")
                    .await?;
            }
        }

        Ok(())
    }
}
