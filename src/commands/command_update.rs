use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Deserialize;
use sqlx::query_scalar;

#[derive(Deserialize)]
struct UpdateCommand {
    category:    String,
    name:        String,
    func:        String,
    arg:         Option<String>,
    return_file: bool,
    ask_day:     bool,
    description: String,
    active:      bool
}

#[post("/command_update")]
pub async fn command_update(state: Data<AppState>,req: HttpRequest, update_command: web::Json<UpdateCommand>) -> impl Responder {
    let update_command = update_command.into_inner();
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
    let command_guid:String = query_scalar(
        r#"
        UPDATE commands
            set category = $2,
            name = $3,
            func = $4,
            arg = $5,
            return_file = $6,
            ask_day = $7,
            description = $8,
            active = $9
        where cast(guid as varchar) = $1
        RETURNING cast(guid as varchar)
        "#)
    .bind(update_command.guid)
    .bind(update_command.category)
    .bind(update_command.name)
    .bind(update_command.func)
    .bind(update_command.arg)
    .bind(update_command.return_file)
    .bind(update_command.ask_day)
    .bind(update_command.description)
    .bind(update_command.active)
    .fetch_one(&state.db)
    .await
    .expect("Failed to update command");

    HttpResponse::Created().json(serde_json::json!({
        "message": "Command update successfully",
        "command_guid": command_guid
    }))
}
