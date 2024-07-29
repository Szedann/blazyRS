use serenity::all::{Context, EventHandler, Message, MessageUpdateEvent};
use crate::app_modules::AppModule;
use crate::commands::commands;

struct Handler;

impl EventHandler for Handler{
    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        println!("{}: {}", new.unwrap().author.name, new.unwrap().content)
    }
}

pub const TEST_MODULE:AppModule = AppModule {
    event_handler: Handler,
    commands: vec![]
};