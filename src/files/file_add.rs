use actix_web::{post, Responder, HttpResponse, HttpRequest};
use actix_web::web::{Data, Payload, Bytes};
use std::fs::File;
use std::io::Write;
use crate::AppState;
use sqlx::query_scalar;

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
    let guid: String = query_scalar(r#"SELECT cast(guid as varchar) FROM users WHERE token = $1"#)
    .bind(token)
    .fetch_one(&state.db)
    .await
    .expect("Failed to execute query");

 // Create a new file in the /tmp directory
    let tmp_path = std::path::Path::new("/tmp/").join(format!("{}.upload", guid));
    let mut tmp_file = match File::create(&tmp_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create file: {}", e)),
    };

    // Read the entire payload into a Bytes type
    let bytes:Bytes = match payload.to_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error reading payload: {}", e)),
    };

    // Write the payload data to the file
    if let Err(e) = tmp_file.write_all(&bytes) {
        return HttpResponse::InternalServerError().body(format!("Error writing to file: {}", e));
    }

    // Return a successful response
    HttpResponse::Ok().finish()
}

