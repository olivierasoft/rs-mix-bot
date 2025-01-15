pub trait Environment {
    fn as_str(&self) -> &'static str;
}

pub enum DiscordEnv {
    DiscordToken,
}

pub enum MixEvents {
    JoinQueue,
}

impl Environment for MixEvents {
    fn as_str(&self) -> &'static str {
        match self {
            Self::JoinQueue => "JOIN_QUEUE",
        }
    }
}

impl Environment for DiscordEnv {
    fn as_str(&self) -> &'static str {
        match self {
            Self::DiscordToken => "DISCORD_TOKEN",
        }
    }
}
