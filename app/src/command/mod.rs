mod emojify;
pub use emojify::*;

use crate::{error::AppError, Context, PrefixContext};

/// このBotの招待URLを表示します
#[poise::command(prefix_command, slash_command)]
pub async fn invite(
    ctx: Context<'_>,
) -> Result<(), AppError> {
    poise::say_reply(ctx, &ctx.data().config.application.invite_url).await?;
    Ok(())
}

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: PrefixContext<'_>, #[flag] global: bool) -> Result<(), AppError> {
    poise::defaults::register_application_commands(ctx.into(), global).await?;

    Ok(())
}
