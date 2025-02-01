use crate::ColumnRef::Column;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Discord::Table)
                    .if_not_exists()
                    .col(string(Discord::Id).primary_key().unique_key())
                    .col(string(Discord::Name))
                    .col(ColumnDef::new(Discord::GlobalName).string())
                    .col(ColumnDef::new(Discord::Email).string())
                    .col(ColumnDef::new(Discord::Discriminator).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-discord-id")
                            .from(User::Table, User::Id)
                            .to(Discord::Table, Discord::Id),
                    )
                    .col(string(User::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(pk_auto(Team::Id))
                    .col(string(Team::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserTeam::Table)
                    .if_not_exists()
                    .col(string(UserTeam::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-team-user-id")
                            .from(UserTeam::Table, UserTeam::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(integer(UserTeam::TeamId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-team-team-id")
                            .from(UserTeam::Table, UserTeam::TeamId)
                            .to(Team::Table, Team::Id),
                    )
                    .primary_key(Index::create().col(UserTeam::TeamId).col(UserTeam::UserId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(string(Game::Name))
                    .col(ColumnDef::new(Game::GuildId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-guild-id")
                            .from(Game::Table, Game::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .col(ColumnDef::new(Game::HomeTeamId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-home-team-id")
                            .from(Game::Table, Game::HomeTeamId)
                            .to(Team::Table, Team::Id),
                    )
                    .col(ColumnDef::new(Game::AwayTeamId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-away-team-id")
                            .from(Game::Table, Game::AwayTeamId)
                            .to(Team::Table, Team::Id),
                    )
                    .col(date_time(Game::Date))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(string(Guild::Id).primary_key().unique_key())
                    .col(string(Guild::Name))
                    .col(ColumnDef::new(Guild::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildUser::Table)
                    .if_not_exists()
                    .col(string(GuildUser::GuildId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-guild-user-guild-id")
                            .from(GuildUser::Table, GuildUser::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .col(string(GuildUser::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-guild-user-user-id")
                            .from(GuildUser::Table, GuildUser::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(date_time(GuildUser::CreatedAt))
                    .primary_key(
                        Index::create()
                            .col(GuildUser::GuildId)
                            .col(GuildUser::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        // Queue: tabela para filas
        manager
            .create_table(
                Table::create()
                    .table(Queue::Table)
                    .if_not_exists()
                    .col(string(Queue::Id).unique_key().primary_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-queue-guild-id")
                            .from(Queue::Table, Queue::Id)
                            .to(Guild::Table, Guild::Id),
                    )
                    .col(integer(Queue::Length))
                    .to_owned(),
            )
            .await?;

        // QueueUser: tabela de relacionamento entre Queue e User
        manager
            .create_table(
                Table::create()
                    .table(QueueUser::Table)
                    .if_not_exists()
                    .col(string(QueueUser::QueueId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-queue-user-queue-id")
                            .from(QueueUser::Table, QueueUser::QueueId)
                            .to(Queue::Table, Queue::Id),
                    )
                    .col(string(QueueUser::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-queue-user-user-id")
                            .from(QueueUser::Table, QueueUser::UserId)
                            .to(User::Table, User::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(QueueUser::QueueId)
                            .col(QueueUser::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserTeam::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Discord::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(QueueUser::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Queue::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GuildUser::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum Game {
    Id,
    Table,
    Date,
    Name,
    GuildId,
    HomeTeamId,
    AwayTeamId,
}
#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum UserTeam {
    Table,
    UserId,
    TeamId,
}
#[derive(DeriveIden)]
enum Discord {
    Table,
    Id,
    Name,
    GlobalName,
    Email,
    Discriminator,
}
#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
    Description,
}
#[derive(DeriveIden)]
enum GuildUser {
    Table,
    GuildId,
    UserId,
    CreatedAt,
}
#[derive(DeriveIden)]
enum Queue {
    Table,
    DiscordId,
    Id,
    Length,
}

#[derive(DeriveIden)]
enum QueueUser {
    Table,
    QueueId,
    UserId,
}
