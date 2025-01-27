use rocket::{serde::Serialize, serde::Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, FromRow, Deserialize)]
pub struct User {
    pub guid: String,
    pub tg_id: i64,
    pub username: String,
    pub password_hash: String,
    pub fio: String,
    pub date_create: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub group: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

