use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub oauth_issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redis_host: String,
    pub redis_port: String,
}

pub fn init() -> AppConfig {
    return AppConfig {
        oauth_issuer: env::var("OAUTH_ISSUER").expect("OAUTH_ISSUER is required"),
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID is required"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET is required"),
        redis_host: env::var("REDIS_HOST").expect("REDIS_HOST is required"),
        redis_port: env::var("REDIS_PORT").expect("REDIS_PORT is required"),
    };
}
