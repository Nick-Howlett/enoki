use axum::{
    extract::State,
    routing::{get},
    Router,
    Json,
    http
};
use http::{header, Method};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use crate::handlers;
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_status = match sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
    })
}

pub fn configure_cors() -> CorsLayer {
    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .parse::<http::HeaderValue>()
        .expect("Invalid ALLOWED_ORIGIN");

    CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
}

pub fn create_router(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/users", get(handlers::user::list_users))
        .layer(cors)
        .with_state(state)
}
