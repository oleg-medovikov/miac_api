use rocket::routes;

mod user_login;

pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        user_login::user_login,
    ]
}
