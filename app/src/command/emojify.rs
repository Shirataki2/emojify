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
            Color::Red => "#FD4028",
            Color::Blue => "#3e44f7",
            Color::Green => "#33f54b",
            Color::Yellow => "#eaff33",
            Color::Purple => "#a31ce0",
            Color::Gray => "#808080",
            Color::Brown => "#A54f4f",
            Color::Aqua => "#44f3f3",
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
    if text.len() > 60 {
        poise::say_reply(ctx, "文字が長すぎます").await?;
        return Ok(());
    }
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
    let msg = poise::send_reply(ctx, |m| m.content("作成中...")).await?;
    if let Ok(mut msg) = msg.message().await {
        msg.edit(ctx.discord(), |m| {
            m.content("作成完了!");
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
    if text.len() > 60 {
        poise::say_reply(ctx, "文字が長すぎます").await?;
        return Ok(());
    }
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
    let msg = poise::send_reply(ctx, |m| m.content("作成中...")).await?;
    if let Ok(mut msg) = msg.message().await {
        msg.edit(ctx.discord(), |m| {
            m.content("作成完了!");
            m.attachment(serenity::AttachmentType::Bytes {
                data: Cow::Owned(buffer),
                filename: "emoji.png".to_string(),
            })
        })
        .await?;
    }
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
    let resp = client
        .get(format!("{}/emoji", config.api.url.as_str()))
        .query(&[
            ("text", text),
            ("font", &font.to_string()),
            ("text_color", text_color),
            ("background_color", background_color),
        ])
        .send()
        .await?;

    if resp.status().is_success() {
        let data = resp.bytes().await?;
        Ok(data.to_vec())
    } else {
        Err(AppError::InvalidColor)
    }
}
