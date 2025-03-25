use rocket::response::content::RawHtml;
use rocket::response::content::RawCss;
use rocket::response::content::RawJavaScript;
use std::fs;
use rocket::get;


#[get("/bot_admin")]
pub fn bot_admin() -> RawHtml<String> {
    let content = fs::read_to_string("static/bot_admin.html").expect("Unable to read file");
    RawHtml(content)
}

#[get("/bot_admin.css")]
pub fn login_css() -> RawCss<String> {
    let content = fs::read_to_string("static/css/bot_admin.css").expect("Unable to read file");
    RawCss(content)
}

#[get("/bot_admin.js")]
pub fn bot_admin_js() -> RawJavaScript<&'static str> {
    let content = fs::read_to_string("static/js/bot_admin.js").expect("Unable to read file");
     RawJavaScript(content)
}

