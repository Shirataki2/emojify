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

/// 使い方の解説を表示します
#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn help(
    ctx: Context<'_>,
) -> Result<(), AppError> {
    let help_text = "入力した文字列から128x128の絵文字として使える画像を生成します．

【使い方】

`/emojify` と入力するとコマンドの候補が表示されます．

2種類のコマンドがありますが色指定の方法が異なっています．
`/emojify simple`では文字の色指定を選択肢から選びます．
`/emojify custom`では文字の色指定をカラーコードで#FF0000のように指定します．
";
    poise::say_reply(ctx, help_text).await?;
    Ok(())
}