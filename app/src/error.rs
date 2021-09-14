use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    ArgParse(#[from] poise::ArgumentParseError),
    #[error("{0}")]
    SlashArg(#[from] poise::SlashArgError),
    #[error("{0}")]
    Internal(#[from] poise::serenity_prelude::Error),
    #[error("{0}")]
    Api(#[from] reqwest::Error),
}
