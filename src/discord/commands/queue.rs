use std::error::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serenity::all::{Channel, Context, Message};
use crate::database::entities::prelude::{User};
use super::super::event::DiscordInstance;

use crate::database::entities::user;

impl DiscordInstance {
    pub async fn queue(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        match msg.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                match User::find_by_id(msg.author.id.get().to_string()).one(self.db.as_ref()).await? {
                    Some(user) => {
                        msg.reply(&ctx.http, format!("User is: {}", user.name)).await?;
                    },
                    None => {
                        // FIX: Error sending message: Exec(SqlxError(Database(SqliteError { code: 1299, message: "NOT NULL constraint failed: user.id" })))
                        user::ActiveModel{name: Set(msg.author.name.to_owned()), ..Default::default()}.insert(self.db.as_ref()).await?;
                        msg.reply(&ctx.http, "Usuário criado").await?;
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