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
    file_bin: String,
}

#[get("/file_get_list")]
pub async fn file_get_list(state: Data<AppState>, req: HttpRequest) -> impl Responder {
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

    let user_guid: Uuid = match find_user {
        Some(user) => user,
        _ => return HttpResponse::NotFound().json("User not found"),
    };

    match query_as::<_, File>(
        r#"
        SELECT
         cast(f.guid as varchar) as file_guid,
         cast(f.time_create as varchar) as time_create,
         f.name as file_name,
         cast(f.file_bin as varchar) as file_bin 
        FROM  files f 
        INNER JOIN  users_x_files u  
        ON(f.guid = u.file and u.user_guid = $1);
           "#,
    )
    .bind(&user_guid)
    .fetch_all(&state.db)
    .await
    {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(_) => HttpResponse::Ok().json("No files found"),
    }
}
