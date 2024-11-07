use anyhow::Result;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::{debug, info};

struct FinkBot {
    db: sqlx::PgPool,
    prefix: &'static str,
}

#[async_trait]
impl EventHandler for FinkBot {
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

    async fn message(&self, ctx: Context, msg: Message) {
        debug!("{}", msg.content);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env
    dotenv::dotenv()?;

    #[cfg(debug_assertions)]
    let max_level = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let max_level = tracing::Level::INFO;

    tracing_subscriber::fmt()
        .pretty()
        .with_line_number(true)
        .with_max_level(max_level)
        .init();

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URLK")?.as_str())
        .await?;

    let bot = FinkBot {
        prefix: "!",
        db: database,
    };

    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents).event_handler(bot).await?;
    client.start().await.unwrap();

    Ok(())
}
