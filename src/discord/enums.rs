pub trait Environment {
    fn as_str(&self) -> &'static str;
}

pub enum DiscordEnv {
    DiscordToken,
}

impl Environment for DiscordEnv {
    fn as_str(&self) -> &'static str {
        match self {
            Self::DiscordToken => "DISCORD_TOKEN",
        }
    }
}
