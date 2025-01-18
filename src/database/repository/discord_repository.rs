use sea_orm::ConnectionTrait;

pub struct DiscordRepository<'a, C>
where
    C: ConnectionTrait,
{
    connection: &'a C,
}

impl<'a, C> DiscordRepository<'a, C>
where
    C: ConnectionTrait,
{
    pub fn new(connection: &'a C) -> Self {
        Self { connection }
    }
}
