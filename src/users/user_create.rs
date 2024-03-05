use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
//use sqlx::{self, FromRow};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize)]
struct NewUser {
    tg_id:       i64,
    username:    String,
    password:    String,
    fio:         String,
    groups:      String,
    description: String,
    active:      bool
}

#[post("/user_create")]
pub async fn user_create(state: Data<AppState>,req: HttpRequest, new_user: web::Json<NewUser>) -> impl Responder {
    let new_user = new_user.into_inner();
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let group = sqlx::query_scalar!(
        r#"
        SELECT groups FROM users
        WHERE token = $1
        "#,
        token
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");

    // Проверяем, является ли пользователь администратором
    if group != "admin"  {
        return HttpResponse::Forbidden().json("Only administrators can create new users");
    }

    // Генерируем хеш пароля
    let hashed_password = hash(new_user.password.as_bytes(), DEFAULT_COST).unwrap();

    // Создаем нового пользователя в базе данных
    let user_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO users (
            tg_id, username, password_hash, fio, groups, description, active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        new_user.tg_id,
        new_user.username,
        hashed_password,
        new_user.fio,
        new_user.groups,
        new_user.description,
        new_user.active
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to create user");

    HttpResponse::Created().json(serde_json::json!({
        "message": "User created successfully",
        "user_id": user_id
    }))
}
