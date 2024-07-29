mod test;

use poise::{Command, serenity_prelude as serenity};
use crate::app_modules::test::TEST_MODULE;
use crate::Context;


pub struct AppModule {
    event_handler: dyn serenity::EventHandler,
    commands: Vec<Command<<Context<'static> as poise::_GetGenerics>::U, <Context<'static> as poise::_GetGenerics>::E>>,
}

pub fn get_modules() -> [AppModule; 1] {
    [TEST_MODULE]
}