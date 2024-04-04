use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{results::AppError, AppState};

#[derive(Serialize, Deserialize)]
pub(crate) struct RawMember {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) is_male: bool,
    pub(crate) sons: Vec<RawMember>,
}

impl RawMember {
    async fn db_create(self) -> Result<(), anyhow::Error> {
        println!("store member");
        Ok(())
    }

    async fn create(
        State(pool): State<AppState>,
        Json(member): Json<Self>,
    ) -> Result<StatusCode, AppError> {
        member.db_create().await?;
        Ok(StatusCode::CREATED)
    }

    async fn db_delete(id: Uuid) -> Result<(), anyhow::Error> {
        println!("delete member with id : {}", id);
        Ok(())
    }

    async fn delete(
        State(pool): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, AppError> {
        Self::db_delete(id).await?;
        Ok(StatusCode::OK)
    }

    async fn db_read(id: Uuid) -> Result<Self, anyhow::Error> {
        println!("read member by id : {}", id);
        Ok(Self {
            id,
            name: String::from("mahmoud"),
            is_male: true,
            sons: vec![],
        })
    }

    async fn read(
        State(pool): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<(StatusCode, Json<RawMember>), AppError> {
        let member = Self::db_read(id).await?;
        Ok((StatusCode::CREATED, Json(member)))
    }

    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/", post(Self::create))
            .route("/:id", delete(Self::delete).get(Self::read))
    }
}
