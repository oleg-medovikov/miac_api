use rocket::routes;

mod users;
mod user_login;


pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        users::get_users,
        user_login::user_login,
    ]
}
