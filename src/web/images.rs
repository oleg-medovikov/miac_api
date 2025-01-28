use rocket::get;
use rocket::fs::NamedFile;
use std::path::Path;

#[get("/image/<filename>")]
pub async fn get_image(filename: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/img/").join(filename)).await.ok()
}
