use super::super::event::DiscordInstance;
use crate::discord::enums::{Environment, MixEvents};
use serenity::all::{
    ButtonStyle, Channel, ChannelType, Context, CreateActionRow, CreateButton, CreateChannel,
    CreateEmbed, CreateEmbedAuthor, CreateMessage, GuildId, Message, PermissionOverwrite,
    PermissionOverwriteType, Permissions,
};
use std::error::Error;

impl DiscordInstance {
    async fn configure_queue_channel(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn Error>> {
        let guild = ctx
            .http
            .get_guild(msg.guild_id.unwrap())
            .await
            .expect("Guild not found");

        let queue_join_channel = CreateChannel::new("entrar-na-fila")
            .kind(ChannelType::Text)
            .permissions(vec![PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(
                    GuildId::new(msg.channel_id.get()).everyone_role(),
                ),
            }]);

        let channel = guild.create_channel(&ctx.http, queue_join_channel).await?;

        let embed = CreateEmbed::new()
            .title("Entrar na fila")
            .description("Para entrar na fila, digite !queue");

        let components = vec![
            CreateButton::new(MixEvents::JoinQueue.as_str())
                .label("Entrar na fila")
                .style(ButtonStyle::Success),
            CreateButton::new(MixEvents::LeftQueue.as_str())
                .label("Sair")
                .style(ButtonStyle::Danger),
        ];

        let button_action_row = CreateActionRow::Buttons(components);

        channel
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .add_embed(embed)
                    .components(vec![button_action_row]),
            )
            .await?;

        msg.reply(
            &ctx.http,
            format!(
                "Channel created with id: {}, you can move wherever you want",
                channel.id
            ),
        )
        .await?;

        Ok(())
    }

    pub async fn configure(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        match msg.channel(&ctx.http).await? {
            Channel::Guild(channel) => {
                self.configure_queue_channel(&ctx, &msg).await?;
            }
            _ => {
                msg.reply(&ctx.http, "This command can only be used in a server")
                    .await?;
            }
        }

        Ok(())
    }
}
