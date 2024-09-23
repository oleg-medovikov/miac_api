use actix_web::{HttpResponse, Responder};
use std::path::Path;
use std::fs;


// Обработчик для корневого маршрута, возвращающий HTML-страницу
pub async fn users() -> impl Responder {
    let html_path = Path::new("static/users.html");
    if let Ok(html) = fs::read_to_string(html_path) {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    } else {
        HttpResponse::InternalServerError().body("Could not read users.html")
    }
}

