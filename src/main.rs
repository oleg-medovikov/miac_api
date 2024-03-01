use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod base;
use base::create_tables::create_tables;
mod services;
use services::{create_user_article, fetch_user_articles, fetch_users};


pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
