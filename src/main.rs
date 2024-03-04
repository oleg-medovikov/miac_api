use actix_web::{web::Data, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{Pool, PgPool, Postgres};

mod users;
use users::login::user_login;
use users::update_password::user_update_password;
use users::user_create::user_create;


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
            //.service(fetch_users)
            //.service(fetch_user_articles)
            //.service(create_user_article)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

