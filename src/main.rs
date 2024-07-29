use crate::app::App;

pub mod commands;
mod app;
mod discord;
mod event_handlers;
mod app_modules;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    discord::setup_discord_client().await;
    let app = App::new().await;


}
