use actix_web::{web, web::Data, post, Responder, HttpResponse};
use serde::Deserialize;
use sqlx::{self,};
use crate::AppState;
use bcrypt::{hash, verify, DEFAULT_COST};


#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/user_login")]
pub async fn user_login(state: Data<AppState>,  credentials: web::Json<Credentials>) -> impl Responder {
    // "POST /user_login".to_string()
    let credentials = credentials.into_inner();

    // Получаем хеш пароля из базы данных
    let user_hash = sqlx::query_scalar!(
        r#"
        SELECT password_hash FROM users
        WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");
    
    let hashed_password = hash(credentials.password.as_bytes(), DEFAULT_COST).unwrap();
    println!("Hashed password: {}", hashed_password);
    
    if let Ok(valid) = verify(credentials.password.as_bytes(), &user_hash) {
        if valid {
                HttpResponse::Ok().json("Logged in successfully")
            } else {
                HttpResponse::Unauthorized().json("Invalid username or password")
            }
        } else {
            HttpResponse::InternalServerError().json("Error verifying password")
        }
}
