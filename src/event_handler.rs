use crate::{
    PError, PContext
};

use crate::FinkBot;

use poise::{
    serenity_prelude::{self as serenity},
    PrefixFrameworkOptions,
};

use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::{info, warn, debug};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, FinkBot, PError>,
    _data: &FinkBot,
) -> Result<(), PError> {
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
                    "[{}] Shard {}/{} ready.",
                    shard.id,
                    shard.id.get() + 1,
                    shard.total
                );
            }
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            debug!(
                "Reaction added! Guild: {:?}, Channel: {:?}, Message: {:?}, User: {:?}",
                add_reaction.guild_id,
                add_reaction.channel_id,
                add_reaction.message_id,
                add_reaction.member
            );
        }
        serenity::FullEvent::ReactionRemove { removed_reaction } => {
            debug!(
                "Reaction removed! Guild: {:?}, Channel: {:?}, Message: {:?}, User: {:?}",
                removed_reaction.guild_id,
                removed_reaction.channel_id,
                removed_reaction.message_id,
                removed_reaction.member
            );
        }
        _ => {
            debug!("Event {} is unimplemented.", event.snake_case_name());
        }
    }

    Ok(())
}
