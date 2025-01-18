use crate::database::entities::prelude::Guild;
use crate::database::entities::{guild, guild_user, queue};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait};

pub struct GuildRepository<'a, C>
where
    C: ConnectionTrait,
{
    connection: &'a C,
}

pub struct GuildWithQueue {
    pub guild: guild::Model,
    pub queue: Option<queue::Model>,
}

impl<'a, C> GuildRepository<'a, C>
where
    C: ConnectionTrait,
{
    pub fn new(connection: &'a C) -> Self {
        Self { connection }
    }

    pub async fn find_guild_queue(&self, guild_id: &String) -> Result<GuildWithQueue, DbErr> {
        let guilds = Guild::find_by_id(guild_id)
            .find_with_related(queue::Entity)
            .all(self.connection)
            .await?;

        let (guild, queues) = guilds
            .first()
            .ok_or_else(|| DbErr::RecordNotFound(guild_id.to_string()))?;

        let guild_with_queue = GuildWithQueue {
            guild: guild.to_owned(),
            queue: queues.first().cloned(),
        };

        Ok(guild_with_queue)
    }

    pub async fn create_guild(
        &self,
        id: String,
        name: String,
        description: Option<String>,
    ) -> Result<guild::Model, DbErr> {
        guild::ActiveModel {
            id: Set(id),
            name: Set(name),
            description: Set(description.to_owned()),
        }
        .insert(self.connection)
        .await
    }

    pub async fn create_guild_queue(&self, guild_id: &String) -> Result<queue::Model, DbErr> {
        Ok(queue::ActiveModel {
            id: Set(guild_id.to_owned()),
            length: Set(0),
        }
        .insert(self.connection)
        .await?)
    }

    pub async fn create_guild_user(
        &self,
        id: &String,
        guild_id: &String,
    ) -> Result<guild_user::Model, DbErr> {
        guild_user::ActiveModel {
            user_id: Set(id.to_owned()),
            guild_id: Set(guild_id.to_owned()),
            created_at: Set(chrono::Utc::now().naive_utc()),
        }
        .insert(self.connection)
        .await
    }
}
