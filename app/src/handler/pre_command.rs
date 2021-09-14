use crate::Context;

pub async fn pre_command(ctx: Context<'_>) {
    let channel_name = ctx
        .channel_id()
        .name(&ctx.discord())
        .await
        .unwrap_or_else(|| "<unknown_channel>".to_string());
    let author = ctx.author().tag();
    match ctx {
        poise::Context::Prefix(ctx) => {
            info!(
                "{} in {} used PREFIX command: {}",
                author, channel_name, &ctx.msg.content
            );
        }
        poise::Context::Application(ctx) => {
            info!(
                "{} in {} used SLASH command '{}'",
                author, channel_name, &ctx.interaction.data.name
            );
        }
    }
}
