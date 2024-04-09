use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub use sqlx::{Pool, Postgres};

pub mod member;
pub async fn get_postgres_pool(db_connection_str: &str) -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_connection_str)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
