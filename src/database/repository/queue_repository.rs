use crate::database::entities::prelude::{*, *};
use crate::database::entities::{queue, queue_user, user};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter,
};

pub struct QueueRepository<'a, C>
where
    C: ConnectionTrait,
{
    connection: &'a C,
}

pub struct QueueWithUsers {
    pub queue: queue::Model,
    pub users: Vec<user::Model>,
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
        QueueUser::delete_many().exec(self.connection).await?;

        Ok(())
    }
    pub async fn count_queue(&self, guild_id: &String) -> Result<u16, DbErr> {
        let count = QueueUser::find()
            .filter(queue_user::Column::QueueId.eq(guild_id))
            .count(self.connection)
            .await?;

        Ok(count as u16)
    }

    pub async fn get_matching_users(&self, guild_id: &String) -> Result<QueueWithUsers, DbErr> {
        let queue_users = Queue::find()
            .filter(queue::Column::Id.eq(guild_id))
            .find_with_related(user::Entity)
            .all(self.connection)
            .await?;

        let (queue, users) = queue_users
            .first()
            .ok_or_else(|| DbErr::RecordNotFound(guild_id.to_string()))?
            .clone();

        Ok(QueueWithUsers { queue, users })
    }

    pub async fn remove_matching_users(&self, queue_with_users: &QueueWithUsers) -> Result<(), DbErr> {
        for user in queue_with_users.users.iter() {
            println!("Guild: {}, User {}", &queue_with_users.queue.id, &user.id);
            QueueUser::delete_by_id((queue_with_users.queue.id.clone(), user.id.clone()))
                .exec(self.connection)
                .await?;
        }
        Ok(())
    }
}
