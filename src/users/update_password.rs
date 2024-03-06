use actix_web::{web, web::Data, put, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
use sqlx::{FromRow, query_as, query};
use bcrypt::{hash, verify, DEFAULT_COST};


#[derive(Deserialize)]
struct PasswordUpdate {
    old_password: String,
    new_password: String,
}

#[derive(FromRow)]
struct User {
    guid: String, 
    password_hash: String,
}

#[put("/user_update_password")]
pub async fn user_update_password(
    state: Data<AppState>,
    req: HttpRequest,
    password_update: web::Json<PasswordUpdate>
) -> impl Responder {
    let password_update = password_update.into_inner();

    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let user = query_as::<_, User>(
        r#"
        SELECT cast(guid as varchar) as guid, password_hash 
        FROM users
        WHERE token = $1
        "#)
    .bind(token)
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");

    // Проверяем старый пароль
    if let Ok(valid) = verify(password_update.old_password.as_bytes(), &user.password_hash) {
        if valid {
            // Генерируем новый хеш пароля
            let new_hashed_password = hash(password_update.new_password.as_bytes(), DEFAULT_COST).unwrap();

            // Обновляем пароль пользователя в базе данных
            query(r#"UPDATE users SET password_hash = $1 WHERE guid = $2"#)
            .bind(new_hashed_password)
            .bind(user.guid)
            .execute(&state.db)
            .await
            .expect("Failed to update user password");

            return HttpResponse::Ok().json("Password updated successfully");
        }
    }

    HttpResponse::Unauthorized().json("Invalid old password")
}
