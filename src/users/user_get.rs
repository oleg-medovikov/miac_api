use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::{FromRow, query_as};

#[derive(Serialize, FromRow)]
struct User {
    pub guid:        String,
    pub tg_id:       i64,
    pub username:    String,
    pub fio:         String,
    pub groups:      String,
    pub description: String,
    pub active:      bool
}

#[get("/user_get")]
pub async fn user_get(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    match query_as::<_, User>("
        SELECT 
            cast(guid as varchar) as guid, tg_id, username, fio, groups, description, active
        FROM users WHERE token::text = $1")
        .bind(token)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::NotFound().json("No users found")
        }
    }
}
