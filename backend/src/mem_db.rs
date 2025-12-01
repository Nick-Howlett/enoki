use redis::aio::ConnectionManager;

pub async fn setup() -> anyhow::Result<ConnectionManager> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    tracing::info!("Connecting to Redis...");
    let client = redis::Client::open(redis_url)?;
    let conn = ConnectionManager::new(client).await?;
    tracing::info!("Redis connection established");

    Ok(conn)
}
