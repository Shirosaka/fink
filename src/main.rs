use database::PostgresPool;
use poise::{
    serenity_prelude::{self as serenity},
    PrefixFrameworkOptions,
};

use serenity::model::prelude::*;
use serenity::prelude::*;

mod models;
mod schema;

mod commands;
mod event_handler;

mod database;

pub(crate) type PError = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type PContext<'a> = poise::Context<'a, FinkBot, PError>;

pub(crate) struct FinkBot {
    pub database: PostgresPool,
    pub prefix: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
            initialize_owners: true,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(bot.prefix.clone()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
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
