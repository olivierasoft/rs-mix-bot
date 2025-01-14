use super::super::event::DiscordInstance;
use crate::database::entities::prelude::User;
use crate::database::entities::{discord, user};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TransactionTrait};
use serenity::all::{Channel, Context, Message};
use std::error::Error;
impl DiscordInstance {
    pub async fn queue_push(
        &self,
        ctx: &Context,
        msg: &Message,
        user: user::Model,
    ) -> Result<(), Box<dyn Error>> {
        {
            let queue_lock = self.queue.lock();
            queue_lock.unwrap().push(user);
        }

        let queue_lock = self.queue.lock();

        msg.reply(
            &ctx.http,
            format!(
                "You placed in queue, has {} users in queue.",
                queue_lock.unwrap().len()
            ),
        )
            .await?;

        Ok(())
    }
    pub async fn join_queue(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        match msg.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                match User::find_by_id(msg.author.id.get().to_string())
                    .one(self.db.as_ref())
                    .await?
                {
                    Some(user) => {
                        self.queue_push(ctx, msg, user).await?;
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

                        self.queue_push(ctx, msg, user).await?;
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
