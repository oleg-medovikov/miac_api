use actix_web::{web::Data, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{Pool, PgPool, Postgres};

mod users;
use users::login::user_login;
use users::update_password::user_update_password;
use users::user_create::user_create;
use users::user_update::user_update;
use users::user_get_all::user_get_all;
use users::check_token::check_token;
use users::user_get::user_get;
use users::drop_token::drop_token;

mod commands;
use commands::command_get_all::command_get_all;
use commands::command_create::command_create;
use commands::command_update::command_update;

mod access;
use access::access_get_all::access_get_all;
use access::access_create::access_create;

mod dirs;
use dirs::dir_create::dir_create;
use dirs::dir_get_all::dir_get_all;
use dirs::dir_update::dir_update;

mod files;
use files::file_add::file_add;

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Не удалось подключиться к базе");
    // Выполняем миграции
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Не удалось создать таблицы");

    let allowed_url = std::env::var("ALLOWED_URL").expect("ALLOWED_URL must be set");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(user_login)
            .service(user_update_password)
            .service(user_create)
            .service(user_update)
            .service(user_get_all)
            .service(check_token)
            .service(drop_token)
            .service(user_get)
            .service(command_get_all)
            .service(command_create)
            .service(command_update)
            .service(access_get_all)
            .service(access_create)
            .service(dir_create)
            .service(dir_get_all)
            .service(dir_update)
            .service(file_add)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
