use ::serenity::all::{GatewayIntents, GuildId};
use poise::serenity_prelude as serenity;
use std::env;

pub mod commands;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = GuildId::new(
        env::var("GUILD_ID")
            .expect("Expected guild id")
            .parse()
            .expect("Guild id is the wrong format"),
    );
    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
