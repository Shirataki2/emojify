use std::borrow::Cow;

use crate::{config::Config, error::AppError, Context};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::SlashChoiceParameter)]
pub enum Font {
    #[name = "明朝体"]
    Mincho,
    #[name = "ゴシック体"]
    Gothic,
    #[name = "太明朝体"]
    BlackMincho,
    #[name = "太ゴシック体"]
    BlackGothic,
    #[name = "丸ゴシック体"]
    MaruGothic,
}

impl ToString for Font {
    fn to_string(&self) -> String {
        let name = match self {
            Font::Mincho => "Mincho",
            Font::Gothic => "Gothic",
            Font::BlackMincho => "BlackMincho",
            Font::BlackGothic => "BlackGothic",
            Font::MaruGothic => "MaruGothic",
        };
        name.to_string()
    }
}

#[derive(Debug, poise::SlashChoiceParameter)]
pub enum Color {
    #[name = "白"]
    White,
    #[name = "黒"]
    Black,
    #[name = "赤"]
    Red,
    #[name = "青"]
    Blue,
    #[name = "緑"]
    Green,
    #[name = "黄"]
    Yellow,
    #[name = "紫"]
    Purple,
    #[name = "灰"]
    Gray,
    #[name = "茶"]
    Brown,
    #[name = "水色"]
    Aqua,
    #[name = "透明"]
    Transparent,
}

impl Color {
    pub fn to_colorcode(&self) -> String {
        let code = match self {
            Color::White => "#FFFFFF",
            Color::Black => "#000000",
            Color::Red => "#FF0000",
            Color::Blue => "#0000FF",
            Color::Green => "#00FF00",
            Color::Yellow => "#FFFF00",
            Color::Purple => "#FF00FF",
            Color::Gray => "#808080",
            Color::Brown => "#A52A2A",
            Color::Aqua => "#00FFFF",
            Color::Transparent => "#00000000",
        };
        code.to_string()
    }
}

/// 指定した文字を絵文字化します
#[poise::command(prefix_command, slash_command)]
pub async fn emojify(_ctx: Context<'_>) -> Result<(), AppError> {
    Ok(())
}

/// 指定した文字を絵文字化します
#[poise::command(prefix_command, slash_command)]
pub async fn simple(
    ctx: Context<'_>,
    #[description = "絵文字化する文字 ( _ を入力することで改行することができます)"]
    text: String,
    #[description = "使用するフォントの種類"] font: poise::Wrapper<Font>,
    #[description = "文字の色"] text_color: Option<poise::Wrapper<Color>>,
    #[description = "文字の背景色"] background_color: Option<poise::Wrapper<Color>>,
) -> Result<(), AppError> {
    let font = font.0;
    let text_color = text_color
        .map(|c| c.0.to_colorcode())
        .unwrap_or_else(|| Color::Red.to_colorcode());
    let background_color = background_color
        .map(|c| c.0.to_colorcode())
        .unwrap_or_else(|| Color::Transparent.to_colorcode());
    let buffer = get_emoji_data(
        &ctx.data().client,
        &ctx.data().config,
        &text,
        &font,
        &text_color,
        &background_color,
    )
    .await?;
    let msg = poise::send_reply(ctx, |m| m.content(text.replace("_", ""))).await?;
    if let Ok(mut msg) = msg.message().await {
        msg.edit(ctx.discord(), |m| {
            m.attachment(serenity::AttachmentType::Bytes {
                data: Cow::Owned(buffer),
                filename: "emoji.png".to_string(),
            })
        })
        .await?;
    }
    Ok(())
}

/// 指定した文字をカラーコードを指定して絵文字化します
#[poise::command(prefix_command, slash_command)]
pub async fn custom(
    ctx: Context<'_>,
    #[description = "絵文字化する文字 ( _ を入力することで改行することができます)"]
    text: String,
    #[description = "使用するフォントの種類"] font: poise::Wrapper<Font>,
    #[description = "文字の色 (#FF0000のように指定)"] text_color: Option<String>,
    #[description = "文字の背景色 (#00000000 とすることで透過背景を使用します)"]
    background_color: Option<String>,
) -> Result<(), AppError> {
    let font = font.0;
    let text_color = text_color.unwrap_or_else(|| Color::Red.to_colorcode());
    let background_color = background_color.unwrap_or_else(|| Color::Transparent.to_colorcode());
    let buffer = get_emoji_data(
        &ctx.data().client,
        &ctx.data().config,
        &text,
        &font,
        &text_color,
        &background_color,
    )
    .await?;
    poise::send_reply(ctx, |m| {
        m.attachment(serenity::AttachmentType::Bytes {
            data: Cow::Owned(buffer),
            filename: "emoji.png".to_string(),
        })
    })
    .await?;
    Ok(())
}

async fn get_emoji_data(
    client: &reqwest::Client,
    config: &Config,
    text: &str,
    font: &Font,
    text_color: &str,
    background_color: &str,
) -> Result<Vec<u8>, AppError> {
    let data = client
        .get(format!("{}/emoji", config.api.url.as_str()))
        .query(&[
            ("text", text),
            ("font", &font.to_string()),
            ("text_color", text_color),
            ("background_color", background_color),
        ])
        .send()
        .await?
        .bytes()
        .await?;
    Ok(data.to_vec())
}