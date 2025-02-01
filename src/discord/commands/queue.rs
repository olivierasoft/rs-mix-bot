use super::super::event::DiscordInstance;
use crate::database::entities::prelude::User;
use crate::database::entities::{discord, user};
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::guild_repository::GuildRepository;
use crate::database::repository::queue_repository::QueueRepository;
use crate::discord::commands::guild::verify_guild;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use serenity::all::{
    Channel, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, Message,
};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

async fn remove_message<'a>(ctx: &Context, msg: &'a Message) -> Result<(), Box<dyn Error>> {
    let ctx = Arc::new(ctx.clone());
    let msg = Arc::new(msg.clone());

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;

        if let Err(err) = msg.delete(&ctx.http).await {
            eprintln!("Failed to delete message: {:?}", err);
        }
    });

    Ok(())
}

async fn create_match(
    db: &DatabaseConnection,
    interaction: &ComponentInteraction,
    ctx: &Context,
) -> Result<(), Box<dyn Error>> {
    let queue_repository = QueueRepository::new(db);

    let mut matching_users = queue_repository
        .get_matching_users(&interaction.guild_id.unwrap().to_string())
        .await?;

    let txn = db.begin().await?;

    let queue_repository = QueueRepository::new(&txn);

    queue_repository
        .remove_matching_users(&matching_users)
        .await?;

    {
        let mut rng = thread_rng();

        &matching_users.users.shuffle(&mut rng);
    }

    let queue_size = 6;

    let home_players = &matching_users.users[0..queue_size / 2].to_vec();
    let away_players = &matching_users.users[queue_size / 2..queue_size].to_vec();
    let game_name = format!("{} TEAM vs {} TEAM", &home_players[0].name, &away_players[0].name);

    for user in home_players.iter() {
        println!("Queue {}", &user.name)
    }

    for user in away_players.iter() {
        println!("Queue {}", &user.name)
    }

    GameRepository::new(&txn)
        .create_game_match_txn(
            &matching_users.queue.id,
            &home_players[0].name,
            &away_players[0].name,
            &game_name,
            home_players,
            away_players,
        )
        .await?;

    txn.commit().await?;

    let mut embed_message = String::new();

    embed_message.push_str(&format!("Game: {} \n\n\n", &game_name.to_uppercase()));

    for player in home_players.iter() {
        embed_message.push_str(&format!("<@{}> \n", &player.id));
    }

    embed_message.push_str("\nVS\n\n");

    for player in away_players.iter() {
        embed_message.push_str(&format!("<@{}> \n", &player.id));
    }

    embed_message.push_str("\n\nAs salas serão criadas automaticamente (e removidas após um tempo), deseja manter a sala pública ou privada?");

    let embed = CreateEmbed::new()
        .title("Partida encontrada")
        .description(embed_message);


    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await?;

    {
        let ctx = Arc::new(ctx.clone());
        let interaction = Arc::new(interaction.clone());

        tokio::spawn(async move {
            sleep(Duration::from_secs(40)).await;

            interaction.delete_response(&ctx.http).await.expect("Failed to delete message");
        });
    }

    Ok(())
}

async fn add_user_to_queue(
    db: &DatabaseConnection,
    interaction: &ComponentInteraction,
    ctx: &Context,
    queue_id: &String,
    user_id: &String,
) -> Result<(), Box<dyn Error>> {
    let queue_repository = QueueRepository::new(db);

    if let Err(_) = queue_repository.push_to_queue(queue_id, user_id).await {
        let interaction_response = CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content("You are already in the queue, wait for start...");

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(interaction_response),
            )
            .await?;

        return Ok(());
    }

    let count = queue_repository.count_queue(queue_id).await?;

    let message = interaction
        .message
        .reply(
            &ctx.http,
            &format!(
                "User <@{}> entered in queue, has {} users in queue",
                user_id, count
            ),
        )
        .await?;

    if count >= 6 {
        remove_message(&ctx, &message).await?;
        create_match(&db, interaction, ctx).await?;

        return Ok(());
    }

    interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await?;

    remove_message(&ctx, &message).await?;

    Ok(())
}

impl DiscordInstance {
    pub async fn queue_push(
        &self,
        interaction: &ComponentInteraction,
        ctx: &Context,
        user: user::Model,
    ) -> Result<(), Box<dyn Error>> {
        let guild_repository = GuildRepository::new(self.db.as_ref());

        let guild_id = interaction.guild_id.unwrap();

        let guild_with_queue = guild_repository
            .find_guild_queue(&guild_id.to_string())
            .await;

        let guild_with_queue = match guild_with_queue {
            Ok(value) => value,
            Err(_) => {
                guild_repository
                    .find_guild_queue(&guild_id.to_string())
                    .await?
            }
        };

        verify_guild(self.db.as_ref(), &ctx, guild_id, interaction.user.clone()).await?;

        if let Some(_) = guild_with_queue.queue {
            add_user_to_queue(
                &self.db.as_ref(),
                interaction,
                ctx,
                &guild_id.to_string(),
                &user.id.to_string(),
            )
                .await?;
        } else {
            guild_repository
                .create_guild_queue(&guild_with_queue.guild.id)
                .await?;

            add_user_to_queue(
                &self.db.as_ref(),
                interaction,
                ctx,
                &guild_with_queue.guild.id,
                &user.id.to_string(),
            )
                .await?;
        }

        Ok(())
    }

    pub async fn check_if_user_is_in_queue() {}
    pub async fn join_queue(
        &self,
        interaction: &ComponentInteraction,
        ctx: &Context,
    ) -> Result<(), Box<dyn Error>> {
        match interaction.message.channel(&ctx.http).await? {
            Channel::Guild(_) => {
                let guild_id = interaction.guild_id.expect("GuildId not found");

                match User::find_by_id(interaction.user.id.get().to_string())
                    .one(self.db.as_ref())
                    .await?
                {
                    Some(user) => {
                        self.queue_push(&interaction, ctx, user).await?;
                    }
                    None => {
                        let txn = self.db.as_ref().begin().await?;

                        let id = Set(interaction.user.id.get().to_string());

                        discord::ActiveModel {
                            id: id.clone(),
                            name: Set(interaction.user.name.to_owned()),
                            global_name: Set(interaction.user.global_name.to_owned()),
                            email: Set(interaction.user.email.to_owned()),
                            discriminator: Set(interaction
                                .user
                                .discriminator
                                .map(|n| n.to_string())),
                        }
                            .insert(&txn)
                            .await?;

                        let user = user::ActiveModel {
                            id: id.clone(),
                            name: Set(interaction.user.name.to_owned()),
                        }
                            .insert(&txn)
                            .await?;

                        if let Err(_) = txn.commit().await {
                            interaction
                                .message
                                .reply(&ctx.http, "User creation failed, try again...")
                                .await?;
                        }

                        verify_guild(self.db.as_ref(), &ctx, guild_id, interaction.user.clone())
                            .await?;

                        self.queue_push(&interaction, ctx, user).await?;
                    }
                }
            }
            _ => {
                interaction
                    .message
                    .reply(&ctx.http, "This command can only be used in a server")
                    .await?;
            }
        }

        Ok(())
    }
}
