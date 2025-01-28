use rocket::routes;

mod xp_alert;
mod login;
mod images;

pub fn web_routes() -> Vec<rocket::Route> {
    routes![
        login::login,
        login::login_css,
        login::login_js,
        login::login_check_js,
        xp_alert::xp_alert_js,
        images::get_image,
    ]
}
