pub struct Target {
    pub website: String,
    pub content_denoting_availability: String
}

pub fn is_username_available(target: &Target, username: &str) -> bool {
    let response = reqwest::blocking::get(target.website.clone() + username).unwrap().text().unwrap();
    response.contains(&target.content_denoting_availability)
}