use crate::db::User;
use anyhow::Ok;
use sqlx::{query, query_as, Pool, Postgres};

use uuid::Uuid;

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

pub async fn delete(pool: Pool<Postgres>, id: Uuid) -> anyhow::Result<()> {
    query!("delete from users where id = $1", id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn read(pool: &Pool<Postgres>, id: Uuid) -> anyhow::Result<User> {
    let user = query_as!(User, "select * from users where id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
