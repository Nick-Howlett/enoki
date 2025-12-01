use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;
use crate::{auth, AppState};

pub async fn log_request(
    State(state): State<AppState>,
    cookies: Cookies,
    request: Request,
    next: Next,
) -> Response {
    if let Some(session_cookie) = cookies.get(auth::SESSION_COOKIE_NAME) {
        let mut redis_conn = state.redis.clone();
        if let Ok(Some(user_id)) = auth::get_session_user_id(&mut redis_conn, session_cookie.value()).await {
            tracing::info!(user_id = %user_id, "authenticated request");
        }
    }

    next.run(request).await
}

pub async fn require_auth(
    State(state): State<AppState>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let session_cookie = cookies
        .get(auth::SESSION_COOKIE_NAME)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut redis_conn = state.redis.clone();
    let user_id = auth::get_session_user_id(&mut redis_conn, session_cookie.value())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to get session user id in require_auth");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}
