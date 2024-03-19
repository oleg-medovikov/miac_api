use actix_web::{web, web::Data, web:Bytes, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::query_scalar;


#[derive(Deserialize)]
struct NewFile {
    filename:    String,
    f
    active:      bool
}

#[post("/file_add")]
pub async fn file_add(state: Data<AppState>,req: HttpRequest, new_user: web::Json<NewUser>) -> impl Responder {
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
    let hashed_password = hash("123".as_bytes(), DEFAULT_COST).unwrap();

    // Создаем нового пользователя в базе данных
    let user_guid:String = query_scalar(
        r#"
        INSERT INTO users (
            tg_id, username, password_hash, fio, groups, description, active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING cast(guid as varchar)
        "#)
    .bind(new_user.tg_id)
    .bind(new_user.username)
    .bind(hashed_password)
    .bind(new_user.fio)
    .bind(new_user.groups)
    .bind(new_user.description)
    .bind(new_user.active)
    .fetch_one(&state.db)
    .await
    .expect("Failed to create user");

    HttpResponse::Created().json(serde_json::json!({
        "message": "User created successfully",
        "user_guid": user_guid
    }))
}
