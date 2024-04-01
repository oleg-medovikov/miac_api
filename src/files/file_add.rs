use crate::AppState;
use actix_web::web::{Bytes, Data, Payload};
use actix_web::{post, HttpRequest, HttpResponse, Responder};
use hex;
use serde_json::json;
use sha3::{Digest, Keccak256};
use sqlx::{query, query_scalar, types::Uuid};

#[post("/file_add")]
pub async fn file_add(state: Data<AppState>, req: HttpRequest, payload: Payload) -> impl Responder {
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

    // Получаем имя файла из заголовка запроса
    let file_name = match req.headers().get("X-File-Name") {
        Some(header_value) => match header_value.to_str() {
            Ok(file_name) => file_name.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid file name"),
        },
        None => return HttpResponse::BadRequest().json("File name not provided"),
    };

    // Читаем весь payload в тип Bytes
    let bytes: Bytes = match payload.to_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading payload: {}", e))
        }
    };

    // Инициализируем Sha256
    let mut hasher = Keccak256::new();
    // Добавляем данные payload в хешер
    hasher.update(&bytes);

    // Получаем итоговый хеш
    let sha256_result = hasher.finalize();
    let sha256_str = hex::encode(sha256_result);

    // пробуем найти бинарник по хешу
    let binary: Option<Uuid> = query_scalar(
        r#"
            SELECT guid from binarys where file_hash = $1
        "#,
    )
    .bind(&sha256_str)
    .fetch_one(&state.db)
    .await
    .ok();

    let binary_guid: Uuid = match binary {
        Some(binary) => binary,
        _ => query_scalar(
            r#"
                INSERT INTO binarys (file_hash, file_data)
                VALUES ($1, $2)
                RETURNING guid;
                "#,
        )
        .bind(&sha256_str)
        .bind(bytes.as_ref())
        .fetch_one(&state.db)
        .await
        .expect("не удалось записать файл"),
    };

    // записываем в таблицу files что мы получили файл
    let file_guid: Uuid = query_scalar(
        r#"
        INSERT INTO FILES (name, file_bin)
        VALUES($1,$2)
        RETURNING guid;
        "#,
    )
    .bind(file_name)
    .bind(&binary_guid)
    .fetch_one(&state.db)
    .await
    .expect("не удалось добавить запись о получении файла");

    // сделать запись, кто именно добавил файл
    query(
        r#"
        INSERT INTO users_x_files (user_guid, file) values($1,$2)
        "#,
    )
    .bind(&user_guid)
    .bind(&file_guid)
    .execute(&state.db)
    .await
    .ok();

    HttpResponse::Ok().json(json!({
        "SHA-256": sha256_str,
        "user_guid": user_guid.to_string(),
        "binary_guid": binary_guid.to_string(),
        "file_guid": file_guid.to_string()
    }))
}
