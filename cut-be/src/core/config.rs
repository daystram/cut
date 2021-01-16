use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub client_secret: String,
    pub redis_host: String,
    pub redis_port: String,
}

pub fn init() -> AppConfig {
    return AppConfig {
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET is required"),
        redis_host: env::var("REDIS_HOST").expect("REDIS_HOST is required"),
        redis_port: env::var("REDIS_PORT").expect("REDIS_PORT is required"),
    };
}
