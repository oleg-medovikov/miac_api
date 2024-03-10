use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::query_scalar;

#[derive(Serialize)]
struct TokenStatus {
    token_valid: bool,
}

#[get("/check_token")]
pub async fn check_token(state: Data<AppState>, req: HttpRequest) -> impl Responder {
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

    let token_status = TokenStatus { token_valid: exists};
    return HttpResponse::Ok().json(token_status)    
}
