use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::{FromRow, query_scalar, query_as};

#[derive(Debug, Serialize, FromRow)]
struct Dir {
    guid:        String,
    name:        String,
    directory:   String,
    description: String,
    active:      bool
}

#[get("/dir_get_all")]
pub async fn dir_get_all(state: Data<AppState>, req: HttpRequest) -> impl Responder {
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

    match query_as::<_, Dir> ("
        SELECT
            cast(guid as varchar) as guid, name, directory, description, active
        FROM dirs")
        .fetch_all(&state.db)
        .await
    {
        Ok(dirs) =>  HttpResponse::Ok().json(dirs),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::NotFound().json("No commands found")
        }
    }
}
