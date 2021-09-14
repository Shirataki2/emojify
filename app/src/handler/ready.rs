use std::sync::Arc;

use poise::serenity_prelude as serenity;

use crate::{config::Config, data::Data, error::AppError};

pub async fn on_ready(
    config: Arc<Config>,
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    _framework: &poise::Framework<Data, AppError>,
) -> Result<Data, AppError> {
    ctx.set_activity(serenity::Activity::listening("/emojify"))
        .await;
    info!("Login as {}", ready.user.name);
    Ok(Data {
        config: config.clone(),
        client: reqwest::Client::new(),
    })
}
