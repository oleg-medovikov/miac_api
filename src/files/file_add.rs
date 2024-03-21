use actix_web::{web::Data, post, Responder, HttpResponse, HttpRequest};
use actix_multipart::Multipart;


#[post("/file_add")]
pub async fn file_add(state: Data<AppState>,req: HttpRequest, mut payload: Multipart) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };


}
