use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::{self, FromRow, query_scalar};

#[derive(Serialize, FromRow)]
struct Command {
    id:          i32,
    category:    String,
    name:        String,
    func:        String,
    arg:         String,
    return_file: bool,
    ask_day:     bool,
    active:      bool
}

#[get("/command_get_all")]
pub async fn command_get_all(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE token = $1)"#)
        .bind(token)
        .fetch_one(&state.db)
        .await
        .expect("Failed to execute query");

    if !exists {
        return HttpResponse::BadRequest().json("Invalid token")
    }

    match sqlx::query_as::<_, Command>(
        "SELECT
            id, category, name, func,
            arg, return_file, ask_day,
            active
         FROM commands")
        .fetch_all(&state.db)
        .await
    {
        Ok(commands) => HttpResponse::Ok().json(commands),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::NotFound().json("No commands found")
        }
    }
}
