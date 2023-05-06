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
        content_denoting_availability: "Sorry, this page isn't available."
    };
}