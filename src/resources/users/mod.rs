use rocket;

pub mod views;
pub mod models;
pub mod urls;

pub fn collect_urls() -> Vec<rocket::Route> {
    return urls::get_urls();
}
