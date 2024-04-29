use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[cfg(feature = "ssr")]
pub mod member;
#[cfg(feature = "ssr")]
pub mod users;
#[cfg(feature = "ssr")]
pub async fn get_postgres_pool(db_connection_str: &str) -> anyhow::Result<Pool<Postgres>> {
    use std::time::Duration;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_connection_str)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub struct RawMember {
    pub id: Uuid,
    pub name: String,
    pub is_male: bool,
    pub sons: Vec<RawMember>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SonlessRawMember {
    pub id: Uuid,
    pub name: String,
    pub is_male: bool,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub member_id: Option<Uuid>,
}
