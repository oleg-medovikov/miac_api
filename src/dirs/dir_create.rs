use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use serde_json::json;
use crate::AppState;
use serde::Deserialize;
use sqlx::query_scalar;

#[derive(Deserialize)]
struct NewDir {
    name:        String,
    directory:   String,
    description: Option<String>,
    active:      bool
}

#[post("/dir_create")]
pub async fn dir_create(state: Data<AppState>,req: HttpRequest, new_dir: web::Json<NewDir>) -> impl Responder {
    let new_dir = new_dir.into_inner();
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
            name, directory, description, active
        )
        VALUES ($1, $2, $3, $4)
        RETURNING cast(guid as varchar);
        "#
    )
    .bind(new_dir.name)
    .bind(new_dir.directory)
    .bind(new_dir.description)
    .bind(new_dir.active)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(result) => { return HttpResponse::Created().json(json!({
            "message": "Директория успешно создана!",
            "dir_guid": result
        }));
        },
        Err(_) => { return HttpResponse::BadRequest().json(json!({
            "message": "Директория с таким именем уже существует!"
        }));
        }
    }
}
