use crate::AppState;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{query_as, query_scalar, types::Uuid, FromRow};

#[derive(Debug, Serialize, FromRow)]
struct File {
    file_guid: String,
    time_create: String,
    file_name: String,
    file_bin: Vec<u8>,
}

#[get("/file_download")]
pub async fn file_download(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let find_user: Option<Uuid> = query_scalar(
        r#"
        SELECT guid FROM users WHERE token = $1
        "#,
    )
    .bind(token)
    .fetch_one(&state.db)
    .await
    .ok();

    match find_user {
        Some(user) => user,
        _ => return HttpResponse::NotFound().json("User not found"),
    };

    // Получаем токен файла из заголовка запроса
    let file_guid: Uuid = match req.headers().get("file_guid") {
        Some(header_value) => match header_value.to_str() {
            Ok(value) => match Uuid::parse_str(value) {
                Ok(token) => token,
                Err(error) => {
                    return HttpResponse::BadRequest().json(format!("Invalid token {}", error))
                }
            },
            Err(_) => return HttpResponse::BadRequest().json("Token not provided"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    match query_as::<_, File>(
        r#"
        SELECT
             cast(f.guid as varchar) as file_guid,
             cast(f.time_create as varchar) as time_create,
             f.name as file_name,
             b.file_data as file_bin 
        FROM  files f 
        INNER JOIN binarys b  
        ON(f.file_bin = b.guid and f.guid = $1);
           "#,
    )
    .bind(&file_guid)
    .fetch_one(&state.db)
    .await
    {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(error) => HttpResponse::Ok().json(format!("File don't found\n {}", error)),
    }
}
