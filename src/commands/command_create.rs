use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use serde_json::json;
use crate::AppState;
use serde::Deserialize;
use sqlx::query_scalar;

#[derive(Deserialize)]
struct NewCommand {
    category:    String,
    name:        String,
    func:        String,
    arg:         Option<String>,
    return_file: bool,
    ask_day:     bool,
    active:      bool
}

#[post("/command_create")]
pub async fn command_create(state: Data<AppState>,req: HttpRequest, new_command: web::Json<NewCommand>) -> impl Responder {
    let new_command = new_command.into_inner();
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let group: String = query_scalar(r#"SELECT groups FROM users WHERE token = $1"#)
    .bind(token)
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");

    // Проверяем, является ли пользователь администратором
    if group != "admin"  {
        return HttpResponse::Forbidden().json("Only administrators can create new commands");
    }

    // Создаем нового пользователя в базе данных
    let result:Result<String, sqlx::Error> = query_scalar(
        r#"
        INSERT INTO commands (
            category, name, func, arg, return_file, ask_day, active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING cast(guid as varchar);
        "#
    )
    .bind(new_command.category)
    .bind(new_command.name)
    .bind(new_command.func)
    .bind(new_command.arg)
    .bind(new_command.return_file)
    .bind(new_command.ask_day)
    .bind(new_command.active)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(result) => { return HttpResponse::Created().json(json!({
            "message": "Команда успешно создана!",
            "command_id": result
        }));
        },
        Err(_) => { return HttpResponse::BadRequest().json(json!({
            "message": "Команда с таким именем уже существует!"
        }));
        }
    }
}
