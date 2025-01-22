use crate::database::entities::prelude::QueueUser;
use crate::database::entities::queue_user;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
};

pub struct QueueRepository<'a, C>
where
    C: ConnectionTrait,
{
    connection: &'a C,
}

impl<'a, C> QueueRepository<'a, C>
where
    C: ConnectionTrait,
{
    pub fn new(connection: &'a C) -> Self {
        Self { connection }
    }

    pub async fn push_to_queue(&self, queue_id: &String, user_id: &String) -> Result<(), DbErr> {
        queue_user::ActiveModel {
            queue_id: Set(queue_id.to_string()),
            user_id: Set(user_id.to_string()),
        }
            .insert(self.connection)
            .await?;

        Ok(())
    }

    pub async fn remove_from_queue(
        &self,
        queue_id: &String,
        user_id: &String,
    ) -> Result<(), DbErr> {
        queue_user::ActiveModel {
            queue_id: Set(queue_id.to_string()),
            user_id: Set(user_id.to_string()),
        }
            .delete(self.connection)
            .await?;

        Ok(())
    }

    pub async fn purge_queue_users(&self) -> Result<(), DbErr> {
        QueueUser::delete_many()
            .exec(self.connection)
            .await?;

        Ok(())
    }
    pub async fn count_queue(&self, guild_id: &String) -> Result<u16, DbErr> {
        let count = QueueUser::find()
            .filter(queue_user::Column::QueueId.eq(guild_id))
            .count(self.connection)
            .await?;

        Ok(count as u16)
    }
}
