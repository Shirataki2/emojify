#[macro_use]
extern crate log;

mod command;
mod config;
mod data;
mod error;
mod handler;

use poise::serenity_prelude as serenity;

pub type Context<'a> = poise::Context<'a, data::Data, error::AppError>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, data::Data, error::AppError>;
pub type ErrorContext<'a> = poise::ErrorContext<'a, data::Data, error::AppError>;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    env_logger::init();

    let config =
        config::load_from_file("./Config.toml").expect("Failed to load config file: `Config.toml`");

    let mut options = poise::FrameworkOptions {
        on_error: |e, ctx| Box::pin(handler::on_error(e, ctx)),
        owners: config.application.owners.clone(),
        pre_command: |ctx| Box::pin(handler::pre_command(ctx)),
        ..Default::default()
    };

    options.command(command::add(), |f| f.category("Miscs"));
    options.command(command::register(), |f| f.category("Miscs"));
    options.command(command::emojify(), |f| {
        f.category("General")
            .subcommand(command::custom(), |f| f)
            .subcommand(command::simple(), |f| f)
    });

    let cloned_config = config.clone();

    let framework = poise::Framework::new(
        config.application.prefix.clone(),
        config.application.application_id,
        move |ctx, ready, framework| {
            Box::pin(handler::on_ready(cloned_config, ctx, ready, framework))
        },
        options,
    );

    if let Err(e) = framework
        .start(
            serenity::ClientBuilder::new(&config.application.token)
                .application_id(config.application.application_id.0)
                .intents(serenity::GatewayIntents::all()),
        )
        .await
    {
        error!("Failed to start application: {:#?}", e);
    }

    Ok(())
}
