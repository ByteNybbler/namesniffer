pub struct Target {
    pub website: &'static str,
    pub content_denoting_availability: &'static str
}

impl Target {
    pub const TWITTER: Target = Target {
        website: "https://twitter.com/",
        content_denoting_availability: "This account doesn"
    };
    pub const DEVIANTART: Target = Target {
        website: "https://deviantart.com/",
        content_denoting_availability: "Page Not Found"
    };
    pub const INSTAGRAM: Target = Target {
        website: "https://instagram.com/",
        content_denoting_availability: "Sorry, this page isn"
    };

    pub fn is_username_available(&self, username: &str) -> bool {
        let response = reqwest::blocking::get(self.website.to_owned() + username).unwrap().text().unwrap();
        response.contains(&self.content_denoting_availability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter() {
        let target = Target::TWITTER;
        assert!(target.is_username_available("sdfyudgsfyusd"));
        assert!(!target.is_username_available("elonmusk"));
    }

    #[test]
    fn test_deviantart() {
        let target = Target::DEVIANTART;
        assert!(target.is_username_available("sdfyudgsfyusd"));
        assert!(!target.is_username_available("lol"));
    }

    #[test]
    fn test_instagram() {
        let target = Target::INSTAGRAM;
        assert!(target.is_username_available("sdfyudgsfyusd"));
        assert!(!target.is_username_available("instagram"));
    }
}