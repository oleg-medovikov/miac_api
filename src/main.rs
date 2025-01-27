use std::env;
use sqlx::PgPool;
use dotenv::dotenv;

mod create_tables;
mod user;
mod models;
mod web;


#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL не задан в конфиге");
    let pool = PgPool::connect(&database_url).await
        .expect("No DB CONNECT!");

    create_tables::create_tables(&pool).await
        .expect("Не удалось создать таблицы");

    rocket::build()
        .mount("/", web::web_routes())
        .mount("/api", user::user_routes())
        .manage(pool)
}
