use std::env;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

const DATABASE_CONNECTOINS: u32 = 1;

pub async fn get_pg_pool() -> Result<Pool<Postgres>> {
    let database_url =
        env::var("DATABASE_URL").context("The DATABASE_URL environment variable must be set")?;

    PgPoolOptions::new()
        .max_connections(DATABASE_CONNECTOINS)
        .connect(&database_url)
        .await
        .context("fail pg pool")
}
