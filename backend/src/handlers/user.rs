use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::user::{CreateUser, User};
use crate::AppState;

pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    User::find_all(&state.db)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!(error = %e, "failed to list users");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    User::find_by_id(&state.db, id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, user_id = %id, "failed to get user by id");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    User::create(&state.db, payload)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|e| {
            tracing::error!(error = %e, "failed to create user");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
