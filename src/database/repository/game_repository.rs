use crate::database::entities::prelude::{*, *};
use crate::database::entities::{game, team, user, user_team};
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr};

pub struct GameRepository<'a, C: ConnectionTrait> {
    connection: &'a C,
}

impl<'a, C: ConnectionTrait> GameRepository<'a, C> {
    pub fn new(connection: &'a C) -> GameRepository<'a, C> {
        Self { connection }
    }

    pub async fn create_game_match_txn(
        &self,
        guild_id: &String,
        home_team_name: &String,
        away_team_name: &String,
        game_name: &String,
        home_team_players: &Vec<user::Model>,
        away_team_players: &Vec<user::Model>,
    ) -> Result<(), DbErr> {
        let home_team = team::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::set(home_team_name.to_owned()),
        }
            .insert(self.connection)
            .await?;

        let away_team = team::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::set(away_team_name.to_owned()),
        }
            .insert(self.connection)
            .await?;

        for player in home_team_players {
            user_team::ActiveModel {
                user_id: ActiveValue::Set(player.id.to_owned()),
                team_id: ActiveValue::Set(home_team.id.to_owned()),
            }
                .insert(self.connection)
                .await?;
        };

        for player in away_team_players {
            user_team::ActiveModel {
                user_id: ActiveValue::Set(player.id.to_owned()),
                team_id: ActiveValue::Set(away_team.id.to_owned()),
            }
                .insert(self.connection)
                .await?;
        };

        game::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(game_name.to_owned()),
            date: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            guild_id: ActiveValue::Set(guild_id.to_owned()),
            home_team_id: ActiveValue::Set(Some(home_team.id.to_owned())),
            away_team_id: ActiveValue::Set(Some(away_team.id.to_owned())),
        }.insert(self.connection).await?;

        Ok(())
    }
}
