use axum::{
    extract::State,
    middleware,
    routing::{get, post},
    Router,
    Json,
    http
};
use http::{header, Method};
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse, DefaultOnFailure},
};
use tracing::Level;

use crate::{auth, handlers};
use crate::{AppState, middleware as app_middleware};

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
        Err(e) => {
            tracing::error!(error = %e, "database health check failed");
            "disconnected"
        }
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
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new()
            .level(Level::INFO)
            .include_headers(false))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_failure(DefaultOnFailure::new().level(Level::ERROR));

    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        .route("/api/users", get(handlers::user::list_users))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::log_request))
        .layer(CookieManagerLayer::new())
        .layer(trace_layer)
        .layer(cors)
        .with_state(state)
}
