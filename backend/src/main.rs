mod auth;
mod db;
mod handlers;
mod middleware;
mod models;
mod mem_db;
mod routes;
mod telemetry;

use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    telemetry::init();

    let db_pool = db::setup().await?;
    let redis_conn = mem_db::setup().await?;

    let state = AppState {
        db: db_pool,
        redis: redis_conn,
    };
    
    let cors = routes::configure_cors();
    let app = routes::create_router(state, cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
