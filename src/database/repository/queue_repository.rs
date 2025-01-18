use sea_orm::ConnectionTrait;

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

    pub fn create_queue(guild_id: &String) {}
}

