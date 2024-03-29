use actix_web::{web, web::Data, post, Responder, HttpResponse};
use serde::Deserialize;
use sqlx::query_scalar;
use crate::AppState;
use bcrypt::verify;
use uuid::Uuid;


#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/user_login")]
pub async fn user_login(state: Data<AppState>,  credentials: web::Json<Credentials>) -> impl Responder {
    // "POST /user_login".to_string()
    let credentials = credentials.into_inner();

    // Ищем пользователя по username
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 and active = true)"#)
        .bind(&credentials.username)
        .fetch_one(&state.db)
        .await
        .expect("Failed to execute query");

    if !exists {
        return HttpResponse::BadRequest().json("Invalid username or password")
    }

    // Получаем хеш пароля из базы данных
    let user_hash: String = query_scalar(r#"SELECT password_hash FROM users WHERE username = $1"#)
    .bind(&credentials.username)
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");
    
    if let Ok(valid) = verify(credentials.password.as_bytes(), &user_hash) {
        if valid {
            
            // Генерируем UUID для токена
            let token = Uuid::new_v4().to_string();

            // Обновляем строку в базе данных с новым токеном
            sqlx::query!(
                r#"
                UPDATE users
                SET token = $1
                WHERE username = $2
                "#,
                token,
                credentials.username
            )
            .execute(&state.db)
            .await
            .expect("Failed to update user token");
            
            // Возвращаем токен в ответе JSON
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Logged in successfully",
                "token": token
            }))

            } else {
                HttpResponse::Unauthorized().json("Invalid username or password")
            }
        } else {
            HttpResponse::InternalServerError().json("Error verifying password")
        }
}
