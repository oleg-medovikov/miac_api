use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::{FromRow, query_scalar, query_as};

#[derive(Debug, Serialize, FromRow)]
struct Access {
    user_guid:    String,
    user_fio:     String,
    command_guid: String,
    command_name: String,
    description:  String
}

#[get("/access_get_all")]
pub async fn access_get_all(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    // Получаем токен пользователя из заголовка запроса
    let token = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return HttpResponse::BadRequest().json("Invalid token"),
        },
        None => return HttpResponse::BadRequest().json("Token not provided"),
    };

    // Ищем пользователя по токену
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE token = $1)"#)
        .bind(token)
        .fetch_one(&state.db)
        .await
        .expect("Failed to execute query");
    
    if !exists {
        return HttpResponse::BadRequest().json("Invalid token")
    }

    match query_as::<_, Access> ("
        SELECT 
            cast(a.client as varchar) as user_guid, u.fio as user_fio,
            cast(a.command as varchar) as command_guid, c.name as command_name,
            a.coment as description
        FROM access a
            inner join users u on(a.client=u.guid)
            inner join commands c on(a.command=c.guid);
        ")
        .fetch_all(&state.db)
        .await
    {
        Ok(accesses) =>  HttpResponse::Ok().json(accesses),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::NotFound().json("No access found")
        }
    }
}
