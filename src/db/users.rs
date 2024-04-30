#[cfg(feature = "ssr")]
use anyhow::Ok;
#[cfg(feature = "ssr")]
use sqlx::{query, query_as, Pool, Postgres};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub member_id: Option<Uuid>,
}

#[cfg(feature = "ssr")]
pub async fn create(
    User {
        id,
        username,
        password,
        member_id,
    }: User,
    pool: &Pool<Postgres>,
) -> anyhow::Result<()> {
    query!(
        r#"
            INSERT INTO users (id,username,password,member_id)
            VALUES($1,$2,$3,$4);
            "#,
        id,
        username,
        password,
        member_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update(
    pool: &Pool<Postgres>,
    User {
        id,
        username,
        password,
        member_id,
    }: User,
) -> anyhow::Result<()> {
    query!(
        r#"
            update users set username = $2,password = $3,member_id = $4 where id = $1
            "#,
        id,
        username,
        password,
        member_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn delete(pool: Pool<Postgres>, id: Uuid) -> anyhow::Result<()> {
    query!("delete from users where id = $1", id)
        .execute(&pool)
        .await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn read(pool: &Pool<Postgres>, id: Uuid) -> anyhow::Result<User> {
    let user = query_as!(User, "select * from users where id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
