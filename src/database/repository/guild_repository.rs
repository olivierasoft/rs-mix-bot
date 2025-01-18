use crate::database::entities::guild;
use crate::database::entities::guild_user;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};

pub struct GuildRepository<'a, C>
where
    C: ConnectionTrait,
{
    connection: &'a C,
}

impl<'a, C> GuildRepository<'a, C>
where
    C: ConnectionTrait,
{
    pub fn new(connection: &'a C) -> Self {
        Self { connection }
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
