use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, user: CreateUser) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, name, password_hash)
            VALUES ($1, $2, '')
            RETURNING id, email, name, password_hash, created_at, updated_at
            "#,
        )
        .bind(&user.email)
        .bind(&user.name)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn create_with_password(
        pool: &sqlx::PgPool,
        user: CreateUser,
        password_hash: String,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, name, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, name, password_hash, created_at, updated_at
            "#,
        )
        .bind(&user.email)
        .bind(&user.name)
        .bind(&password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &sqlx::PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_all(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }
}
