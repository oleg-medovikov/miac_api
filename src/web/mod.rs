use rocket::routes;

mod login;

pub fn web_routes() -> Vec<rocket::Route> {
    routes![
        login::login,
        login::login_css,
        login::login_js,
        login::login_check_js,
    ]
}
