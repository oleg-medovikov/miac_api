use actix_web::{web, web::Data, post, Responder, HttpResponse, HttpRequest};
use serde_json::json;
use crate::AppState;
use serde::Deserialize;
use sqlx::{query_scalar, types::Uuid};


#[derive(Deserialize)]
struct NewAccess {
    user_guid:    String,
    command_guid: String,
    description:  String,
}

#[post("/access_create")]
pub async fn access_create(state: Data<AppState>,req: HttpRequest, new_access: web::Json<NewAccess>) -> impl Responder {
    let new_access = new_access.into_inner();
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

    match (Uuid::parse_str(&new_access.user_guid), Uuid::parse_str(&new_access.command_guid)) {
        (Ok(user_uuid), Ok(command_uuid)) => {
            let result: Result<String, sqlx::Error> = query_scalar(
                r#"
                INSERT INTO access (client, command, coment)
                VALUES ($1, $2, $3)
                ON CONFLICT (client, command) DO UPDATE
                SET coment = EXCLUDED.coment
                RETURNING cast(client as varchar);
                "#
            )
            .bind(user_uuid)
            .bind(command_uuid)
            .bind(new_access.description)
            .fetch_one(&state.db)
            .await;

            match result {
                Ok(result) => HttpResponse::Created().json(json!({
                    "message": "Доступ к команде успешно создан!",
                    "user_guid": result
                })),
                Err(e) => HttpResponse::BadRequest().json(json!({
                    "message": "Ошибка при создании доступа к команде.",
                    "error": e.to_string(),
                })),
            }
        },
        _ => {
            HttpResponse::BadRequest().json(json!({
                "message": "Один из UUID неверного формата.",
            }))
        }
    }
}
