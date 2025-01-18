use sea_orm::ConnectionTrait;

pub struct UserRepository<C>
where
    C: ConnectionTrait,
{
    connection: C,
}

impl<C> UserRepository<C>
where
    C: ConnectionTrait,
{}
