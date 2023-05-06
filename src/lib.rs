mod target;
pub use target::*;

pub fn is_username_available(target: &Target, username: &str) -> bool {
    let response = reqwest::blocking::get(target.website.to_owned() + username).unwrap().text().unwrap();
    response.contains(&target.content_denoting_availability)
}