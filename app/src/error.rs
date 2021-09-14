use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("⚠　引数が不正です\n【詳細】\n{0}")]
    ArgParse(#[from] poise::ArgumentParseError),
    #[error("⚠　スラッシュコマンドの引数が不正です\n【詳細】\n{0}")]
    SlashArg(#[from] poise::SlashArgError),
    #[error("⚠　Discordサーバーとの通信時にエラーが発生しました\n後程お試しください")]
    Internal(#[from] poise::serenity_prelude::Error),
    #[error("⚠　通信エラーが発生しました\n後程お試しください")]
    Api(#[from] reqwest::Error),
    #[error("⚠　色の指定方法に誤りがあります")]
    InvalidColor,
}
