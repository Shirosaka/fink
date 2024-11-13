
use anyhow::Result;

use database::PostgresPool;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};

use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::{debug, info, warn};

mod schema;

mod database;

type Error = Box<dyn std::error::Error + Send + Sync>;
type PContext<'a> = poise::Context<'a, FinkBot, Error>;

struct FinkBot {
    database: PostgresPool,
    prefix: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Program configuration
    // load .env variables (todo: restrict to dev only)
    dotenvy::dotenv()?;
    // setup tracing logger
    // setup_logging()?;
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = database::init_pool(database_url, 10)?;

    let prefix = std::env::var("DISCORD_PREFIX").unwrap_or(String::from("!"));

    let bot = FinkBot {
        database: pool,
        prefix,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // commands: vec![age()],
            initialize_owners: true,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(bot.prefix.clone()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(bot)
            })
        })
        .build();

    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, FinkBot, Error>,
    data: &FinkBot,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready {
            data_about_bot: rdy,
        } => {
            if let Ok(bot_gateway_res) = ctx.http().get_bot_gateway().await {
                info!("{:?}", bot_gateway_res.session_start_limit)
            } else {
                warn!("Failed to fetch bot gateway information.")
            }

            if let Some(shard) = rdy.shard {
                // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
                //
                // This may seem unintuitive, but it models Discord's behaviour.
                info!(
                    "{} is connected on shard {}! Total shards: {}",
                    &rdy.user.name, shard.id, shard.total
                );

                for guild in &rdy.guilds {
                    info!("In guild: {}", guild.id)
                }
            }
        }
        _ => {
            debug!("Event {} is unimplemented.", event.snake_case_name());
        }
    }

    Ok(())
}
