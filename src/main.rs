use actix_web::{web::Data, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, PgPool, Postgres};

mod base;
use base::create_tables::create_tables;
mod users;
use users::login::user_login;


pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    // создаем таблицы
    match create_tables(&pool).await {
        Ok(_) => {
            println!("Tables created successfully");
        },
        Err(err) => {
            eprintln!("Error creating tables: {}", err);
        }
    }

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
            //.service(fetch_users)
            //.service(fetch_user_articles)
            //.service(create_user_article)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
