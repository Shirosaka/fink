use anyhow::Result;

struct BotData {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env
    dotenv::dotenv()?;
    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    Ok(())
}
