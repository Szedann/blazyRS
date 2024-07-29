use std::env;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub struct App {
    pub db: Pool<Postgres>,
}

impl App {
    pub async fn new() -> Self {
        Self {
        db: PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("Missing Required Env Var DATABASE_URL"))
            .await.expect("Unknown error occurred while connecting to DB"),
        }
    }
}