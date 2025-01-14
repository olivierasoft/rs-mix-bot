use super::super::event::DiscordInstance;
use serenity::all::{Channel, ChannelType, Context, CreateChannel, GuildId, Message, PermissionOverwrite, PermissionOverwriteType, Permissions};
use std::error::Error;

impl DiscordInstance {
    async fn configure_queue_channel(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn Error>> {
        let guild = ctx.http.get_guild(msg.guild_id.unwrap()).await.unwrap();

        let queue_join_channel = CreateChannel::new("entrar-na-fila-2")
            .kind(ChannelType::Text)
            .permissions(vec![PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(
                    GuildId::new(msg.channel_id.get()).everyone_role(),
                ),
            }]);

        let channel = guild.create_channel(&ctx.http, queue_join_channel).await?;

        msg.reply(&ctx.http, format!("Channel created with id: {}", channel.id)).await?;

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
