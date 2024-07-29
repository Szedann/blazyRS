use std::any::Any;
use poise::serenity_prelude as serenity;
use std::env;
use std::fmt::Pointer;
use serenity::all::{Context, Message, MessageUpdateEvent};
use crate::{commands, Data, Error};
use crate::app_modules::{AppModule, get_modules};

pub struct Handler;

impl serenity::EventHandler for Handler{
    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        todo!()
    }
}


pub async fn setup_discord_client() {
    // set the bot up
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = serenity::GuildId::new(
        env::var("GUILD_ID")
            .expect("Expected guild id")
            .parse()
            .expect("Guild id is the wrong format"),
    );
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

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

    let client_pre_init = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .event_handler(Handler);

    let modules = get_modules();
    for module in modules {
        todo!()
    }


    let client = client_pre_init.await;
    client.unwrap().start().await.unwrap();
}