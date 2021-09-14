use crate::{ErrorContext, error::AppError};


pub async fn on_error(error: AppError, ctx: ErrorContext<'_>) {
    error!("Command Error:{}", error);
    if let poise::ErrorContext::Command(ctx) = ctx {
        let reply = error.to_string();
        if let Err(e) = poise::say_reply(ctx.ctx(), reply).await {
            error!("Error sending reply: {}", e);
        }
    }
}
