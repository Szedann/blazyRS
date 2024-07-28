use crate::Context;
use poise::Command;

pub mod member;
pub mod ping;

pub fn commands() -> Vec<
    Command<
        <Context<'static> as poise::_GetGenerics>::U,
        <Context<'static> as poise::_GetGenerics>::E,
    >,
> {
    vec![ping::ping(), member::member()]
}
