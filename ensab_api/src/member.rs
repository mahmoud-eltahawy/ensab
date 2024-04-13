use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use uuid::Uuid;

use crate::{results::AppError, AppState};

use db::member::{self, RawMember, SonlessRawMember};

async fn create_fatherless(
    State(state): State<AppState>,
    Json(member): Json<RawMember>,
) -> Result<StatusCode, AppError> {
    let mut transaction = state.pool.begin().await?;
    member::create(member, &mut transaction, None).await?;
    transaction.commit().await?;
    Ok(StatusCode::CREATED)
}

async fn create_sons(
    State(state): State<AppState>,
    Path(parent_id): Path<Uuid>,
    Json(members): Json<Vec<RawMember>>,
) -> Result<StatusCode, AppError> {
    let mut transaction = state.pool.begin().await?;
    for member in members {
        member::create(member, &mut transaction, Some(parent_id)).await?;
    }
    transaction.commit().await?;
    Ok(StatusCode::CREATED)
}

async fn update(
    State(state): State<AppState>,
    Json(members): Json<Vec<SonlessRawMember>>,
) -> Result<StatusCode, AppError> {
    member::update(state.pool, members).await?;
    Ok(StatusCode::OK)
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    member::delete(state.pool, id).await?;
    Ok(StatusCode::OK)
}

async fn read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<RawMember>), AppError> {
    let member = member::read(&state.pool, id).await?;
    Ok((StatusCode::CREATED, Json(member)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_fatherless).put(update))
        .route("/:id", get(read).delete(delete).post(create_sons))
}
