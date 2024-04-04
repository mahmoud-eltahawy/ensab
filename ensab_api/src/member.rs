use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use sqlx::{query, Pool, Postgres, Transaction};
use uuid::Uuid;

use crate::{results::AppError, AppState};
use chrono::{NaiveDateTime, Utc};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawMember {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) is_male: bool,
    pub(crate) sons: Vec<RawMember>,
}

use async_recursion::async_recursion;

impl RawMember {
    #[async_recursion]
    async fn store_member(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        parent_id: Option<Uuid>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let now = NaiveDateTime::new(now.date_naive(), now.time());
        query!(
            r#"
            INSERT INTO "member" (id,"name",is_male,insert_date,parent_id)
            VALUES($1,$2,$3,$4,$5);
            "#,
            self.id,
            self.name,
            self.is_male,
            now,
            parent_id
        )
        .execute(&mut **transaction)
        .await?;
        for son in self.sons {
            son.store_member(transaction, Some(self.id)).await?;
        }

        Ok(())
    }
    async fn db_create(self, pool: Pool<Postgres>) -> Result<(), anyhow::Error> {
        let mut transaction = pool.begin().await?;
        self.store_member(&mut transaction, None).await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn create(
        State(state): State<AppState>,
        Json(member): Json<Self>,
    ) -> Result<StatusCode, AppError> {
        member.db_create(state.pool).await?;
        Ok(StatusCode::CREATED)
    }

    async fn db_delete(id: Uuid) -> Result<(), anyhow::Error> {
        println!("delete member with id : {}", id);
        Ok(())
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, AppError> {
        Self::db_delete(id).await?;
        Ok(StatusCode::OK)
    }

    async fn db_read(pool: Pool<Postgres>, id: Uuid) -> Result<Self, anyhow::Error> {
        println!("read member by id : {}", id);
        let num = query!("select 1 + 1 as sum")
            .fetch_one(&pool)
            .await?
            .sum
            .unwrap();
        Ok(Self {
            id,
            name: format!("mahmoud {}", num),
            is_male: true,
            sons: vec![],
        })
    }

    async fn read(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<(StatusCode, Json<RawMember>), AppError> {
        let member = Self::db_read(state.pool, id).await?;
        Ok((StatusCode::CREATED, Json(member)))
    }

    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/", post(Self::create))
            .route("/:id", delete(Self::delete).get(Self::read))
    }
}
