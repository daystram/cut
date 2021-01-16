use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub client_secret: String,
}

pub fn init() -> AppConfig {
    return AppConfig {
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET is required"),
    };
}
