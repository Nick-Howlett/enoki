use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::models::user::{CreateUser, User};
use crate::AppState;

pub const SESSION_COOKIE_NAME: &str = "session_id";
const SESSION_EXPIRY_SECONDS: u64 = 60 * 60 * 24 * 7;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn create_session(
    redis: &mut redis::aio::ConnectionManager,
    user_id: Uuid,
) -> Result<String, redis::RedisError> {
    let session_id = Uuid::new_v4().to_string();
    redis
        .set_ex::<_, _, ()>(
            &session_id,
            user_id.to_string(),
            SESSION_EXPIRY_SECONDS,
        )
        .await?;
    Ok(session_id)
}

pub async fn get_session_user_id(
    redis: &mut redis::aio::ConnectionManager,
    session_id: &str,
) -> Result<Option<Uuid>, redis::RedisError> {
    let user_id: Option<String> = redis.get(session_id).await?;
    Ok(user_id.and_then(|id| Uuid::parse_str(&id).ok()))
}

pub async fn delete_session(
    redis: &mut redis::aio::ConnectionManager,
    session_id: &str,
) -> Result<(), redis::RedisError> {
    redis.del::<_,()>(session_id).await?;
    Ok(())
}

pub async fn signup(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<SignupRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    let password_hash = hash_password(&payload.password).map_err(|e| {
        tracing::error!(error = %e, "failed to hash password");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = User::create_with_password(
        &state.db,
        CreateUser {
            email: payload.email,
            name: payload.name,
        },
        password_hash,
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to create user");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut redis_conn = state.redis.clone();
    let session_id = create_session(&mut redis_conn, user.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, user_id = %user.id, "failed to create session");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, session_id))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(SESSION_EXPIRY_SECONDS as i64));

    cookies.add(cookie.build());

    Ok((StatusCode::CREATED, Json(AuthResponse { user })))
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = User::find_by_email(&state.db, &payload.email)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, email = %payload.email, "failed to find user by email");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| {
            tracing::error!(error = %e, "failed to verify password");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !is_valid {
        tracing::warn!(email = %payload.email, "invalid password attempt");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut redis_conn = state.redis.clone();
    let session_id = create_session(&mut redis_conn, user.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, user_id = %user.id, "failed to create session");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, session_id))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(SESSION_EXPIRY_SECONDS as i64));

    cookies.add(cookie.build());

    Ok(Json(AuthResponse { user }))
}

pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> StatusCode {
    if let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) {
        let mut redis_conn = state.redis.clone();
        let _ = delete_session(&mut redis_conn, cookie.value()).await;
    }

    cookies.remove(Cookie::from(SESSION_COOKIE_NAME));
    StatusCode::OK
}

pub async fn me(State(state): State<AppState>, cookies: Cookies) -> Result<Json<AuthResponse>, StatusCode> {
    let session_cookie = cookies
        .get(SESSION_COOKIE_NAME)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut redis_conn = state.redis.clone();
    let user_id = get_session_user_id(&mut redis_conn, session_cookie.value())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to get session user id");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = User::find_by_id(&state.db, user_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, user_id = %user_id, "failed to find user by id");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(AuthResponse { user }))
}
