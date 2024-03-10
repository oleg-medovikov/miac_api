use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
use sqlx::query_scalar;

#[derive(Deserialize)]
struct UpdateUser {
    guid:        String,
    tg_id:       i64,
    username:    String,
    fio:         String,
    groups:      String,
    description: String,
    active:      bool
}

#[post("/user_update")]
pub async fn user_update(state: Data<AppState>,req: HttpRequest, update_user: web::Json<UpdateUser>) -> impl Responder {
    let update_user = update_user.into_inner();
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
        return HttpResponse::Forbidden().json("Only administrators can update users");
    }

    // Создаем нового пользователя в базе данных
    let user_guid:String = query_scalar(
        r#"
        UPDATE users
            set tg_id = $2,
            username = $3,
            fio = $4,
            groups = $5,
            description = $6,
            active = $7,
            token = null
        where cast(guid as varchar) = $1
        RETURNING cast(guid as varchar)
        "#)
    .bind(update_user.guid)
    .bind(update_user.tg_id)
    .bind(update_user.username)
    .bind(update_user.fio)
    .bind(update_user.groups)
    .bind(update_user.description)
    .bind(update_user.active)
    .fetch_one(&state.db)
    .await
    .expect("Failed to update user");

    HttpResponse::Created().json(serde_json::json!({
        "message": "User update successfully",
        "user_guid": user_guid
    }))
}
