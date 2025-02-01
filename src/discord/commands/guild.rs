use crate::database::entities::guild;
use crate::database::entities::prelude::*;
use crate::database::repository::guild_repository::GuildRepository;
use sea_orm::{DatabaseConnection, EntityTrait, TransactionTrait};
use serenity::all::{Context, GuildId, User as SerenityUser};
use std::error::Error;

pub async fn verify_guild(
    db: &DatabaseConnection,
    ctx: &Context,
    guild_id: GuildId,
    user: SerenityUser,
) -> Result<guild::Model, Box<dyn Error>> {
    let database_guild = Guild::find_by_id(guild_id.to_string()).one(db).await?;

    let discord_guild = ctx.http.get_guild(guild_id).await?;

    let database_guild = if let Some(guild) = database_guild {
        let guild_repository = GuildRepository::new(db);

        let user_guild = GuildUser::find_by_id((guild_id.to_string(), user.id.to_string()))
            .one(db)
            .await?;

        if let Some(v) = user_guild {
            println!("Has value {}", user.name);
        } else {
            println!("None Value");
            guild_repository.create_guild_user(&user.id.to_string(), &guild.id).await?;
        }


        guild
    } else {
        let txn = db.begin().await?;

        let guild_repository = GuildRepository::new(&txn);

        let guild = guild_repository
            .create_guild(
                discord_guild.id.to_string(),
                discord_guild.name.to_string(),
                discord_guild.description.to_owned(),
            )
            .await?;

        guild_repository.create_guild_user(&user.id.to_string(), &guild.id).await?;

        txn.commit().await?;

        guild
    };

    Ok(database_guild)
}
