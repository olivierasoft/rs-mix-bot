use crate::database::entities::prelude::{*, *};
use crate::database::entities::user;
use sea_orm::ConnectionTrait;

pub struct GameRepository<'a, C: ConnectionTrait> {
    connection: &'a C,
}

impl<'a, C: ConnectionTrait> GameRepository<'a, C> {
    pub fn new(connection: &'a C) -> GameRepository<'a, C> {
        Self { connection }
    }

    pub async fn create_game_match_txn(
        guild_id: &String,
        home_team_players: Vec<user::Model>,
        away_team_players: Vec<user::Model>,
    ) {
        for player in home_team_players {
            // game::ActiveModel {
            //     id: guild_id
            // }
        }
    }
}
