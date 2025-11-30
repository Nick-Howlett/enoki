mod db;
mod handlers;
mod models;
mod routes;
mod telemetry;

use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    telemetry::init();

    let db_pool = db::setup().await?;
    let state = AppState { db: db_pool };

    let cors = routes::configure_cors();
    let app = routes::create_router(state, cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
