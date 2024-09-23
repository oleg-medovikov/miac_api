//use actix_cors::Cors;
use actix_web::{web, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use actix_files;


mod users;
use users::check_token::check_token;
use users::drop_token::drop_token;
use users::login::user_login;
use users::update_password::user_update_password;
use users::user_create::user_create;
use users::user_get::user_get;
use users::user_get_all::user_get_all;
use users::user_update::user_update;

mod commands;
use commands::command_create::command_create;
use commands::command_get_all::command_get_all;
use commands::command_update::command_update;

mod access;
use access::access_create::access_create;
use access::access_delete::access_delete;
use access::access_get_all::access_get_all;

mod dirs;
use dirs::dir_create::dir_create;
use dirs::dir_get_all::dir_get_all;
use dirs::dir_update::dir_update;

mod files;
use files::file_add::file_add;
use files::file_download::file_download;
use files::file_get_list::file_get_list;

mod html;
use html::login::login;
use html::users::users;
use html::commands::commands;
use html::dirs::dirs;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе");
    // Выполняем миграции
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Не удалось создать таблицы");
    
    // CORS более не нужен
    // let allowed_url = std::env::var("ALLOWED_URL").expect("ALLOWED_URL must be set");

    HttpServer::new(move || {
        //let cors = Cors::default()
        //    .allowed_origin(&allowed_url)
        //    .allowed_methods(vec!["GET", "POST"])
        //    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //    .allowed_header(header::CONTENT_TYPE)
        //    .max_age(3600);
        App::new()
            //.wrap(cors)
            .route("/", web::get().to(login))
            .route("/login.html", web::get().to(login))
            .route("/users.html", web::get().to(users))
            .route("/commands.html", web::get().to(commands))
            .route("/dirs.html", web::get().to(dirs))
            .service(
                actix_files::Files::new("/static", "static")
                    .show_files_listing()
                    .index_file("login.html")
                    .index_file("users.html")
                    .index_file("dirs.html")
                    .index_file("commands.html"),
            )
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
            .service(access_delete)
            .service(dir_create)
            .service(dir_get_all)
            .service(dir_update)
            .service(file_add)
            .service(file_get_list)
            .service(file_download)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
