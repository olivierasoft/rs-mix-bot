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
                    .col(pk_auto(Discord::Id))
                    .col(string(Discord::Name))
                    .col(string(Discord::Identifier))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name))
                    .col(ColumnDef::new(User::DiscordId).integer())
                    .foreign_key(ForeignKey::create()
                        .name("fk-user-discord-id")
                        .from(User::Table, User::Id)
                        .to(Discord::Table, Discord::Id)
                    )
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
                    .col(integer(UserTeam::UserId))
                    .col(integer(UserTeam::TeamId))
                    .primary_key(Index::create()
                        .col(UserTeam::TeamId)
                        .col(UserTeam::UserId)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(pk_auto(Match::Id))
                    .col(string(Match::Name))
                    .col(ColumnDef::new(Match::HomeTeamId).integer())
                    .foreign_key(ForeignKey::create()
                        .name("fk-match-home-team-id")
                        .from(Match::Table, Match::HomeTeamId)
                        .to(Team::Table, Team::Id))
                    .col(ColumnDef::new(Match::AwayTeamId).integer())
                    .foreign_key(ForeignKey::create()
                        .name("fk-match-away-team-id")
                        .from(Match::Table, Match::AwayTeamId)
                        .to(Team::Table, Team::Id))
                    .col(date(Match::Date))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Match::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(UserTeam::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Team::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Discord::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    DiscordId,
}
#[derive(DeriveIden)]
enum Match {
    Id,
    Table,
    Date,
    Name,
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
    Name,
    Identifier,
    Id,
}
