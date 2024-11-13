#[allow(unused)] 

use anyhow::Result;

use poise::serenity_prelude as serenity;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::{debug, info, warn};

struct FinkBot {
    prefix: String,
}

#[async_trait]
impl EventHandler for FinkBot {
    async fn message(&self, _: Context, msg: Message) {
        debug!("{}", msg.content);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            info!(
                "{} is connected on shard {}! Total shards: {}",
                ready.user.name, shard.id, shard.total
            );

            for guild in ready.guilds {
                info!("In guild: {}", guild.id)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Program configuration
    // load .env variables (todo: restrict to dev only)
    dotenvy::dotenv()?;
    // setup tracing logger
    // setup_logging()?;
    tracing_subscriber::fmt::init();



    let prefix = std::env::var("DISCORD_PREFIX").unwrap_or(String::from("!"));

    let bot = FinkBot {
        prefix,
    };

    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(&token, intents).event_handler(bot).await?;
    client.start().await?;

    Ok(())
}
