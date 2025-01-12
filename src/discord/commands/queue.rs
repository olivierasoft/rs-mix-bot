use std::error::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TransactionTrait};
use serenity::all::{Channel, Context, Message};
use crate::database::entities::prelude::{User};
use super::super::event::DiscordInstance;

use crate::database::entities::{discord, user};

impl DiscordInstance {
    pub async fn queue(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        match msg.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                match User::find_by_id(msg.author.id.get().to_string()).one(self.db.as_ref()).await? {
                    Some(user) => {
                        msg.reply(&ctx.http, format!("User is: {}", user.name)).await?;
                    },
                    None => {
                        let txn = self.db.as_ref().begin().await?;

                        let id = Set(msg.id.get().to_string());

                        discord::ActiveModel{id: id.clone(), name: Set(msg.author.name.to_owned())}.insert(&txn).await?;
                        user::ActiveModel{id: id.clone(), name: Set(msg.author.name.to_owned())}.insert(&txn).await?;

                        if let Err(_) = txn.commit().await {
                            msg.reply(&ctx.http, "User creation failed, try again...").await?;
                        }

                        msg.reply(&ctx.http, "User created...").await?;
                    }
                }
            },
            _ => {
                msg.reply(&ctx.http, "This command can only be used in a server").await?;
            }
        }

        Ok(())
    }
}