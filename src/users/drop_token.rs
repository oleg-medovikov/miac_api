use actix_web::{web::Data, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;


#[post("/drop_token")]
pub async fn drop_token(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    sqlx::query("UPDATE users SET token = NULL WHERE token = $1")
        .bind(token)
        .execute(&state.db)
        .await
        .expect("Invalid token");

    return HttpResponse::Ok().json("Token deleted")
}
