use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
use sqlx::query_scalar;

#[derive(Deserialize)]
struct UpdateDir {
    guid:        String,
    name:        String,
    directory:   String,
    description: Option<String>,
    active:      bool
}

#[post("/dir_update")]
pub async fn dir_update(state: Data<AppState>,req: HttpRequest, update_dir: web::Json<UpdateDir>) -> impl Responder {
    let update_dir = update_dir.into_inner();
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
        return HttpResponse::Forbidden().json("Only administrators can update command");
    }

    // Создаем нового пользователя в базе данных
    let dir_guid:String = query_scalar(
        r#"
        UPDATE dirs
            set name = $2,
            directory = $3,
            description = $4,
            active = $5
        where cast(guid as varchar) = $1
        RETURNING cast(guid as varchar)
        "#)
    .bind(update_dir.guid)
    .bind(update_dir.name)
    .bind(update_dir.directory)
    .bind(update_dir.description)
    .bind(update_dir.active)
    .fetch_one(&state.db)
    .await
    .expect("Failed to update dir");

    HttpResponse::Created().json(serde_json::json!({
        "message": "Dir update successfully",
        "dir_guid": dir_guid
    }))
}
