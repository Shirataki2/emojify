mod emojify;
pub use emojify::*;

use crate::{error::AppError, Context, PrefixContext};

/// Add two numbers
#[poise::command(prefix_command, slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "1st operand"] a: f32,
    #[description = "2nd operand"] b: f32,
) -> Result<(), AppError> {
    poise::say_reply(ctx, format!("{} + {} = {}", a, b, a + b)).await?;
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
