use actix_web::{App, HttpServer};
use crate::core;
use crate::AppConfig;

pub mod controllers;
pub mod handlers;
pub mod router;

#[derive(Clone)]
pub struct Module {
    pub config: AppConfig,
    pub rd: String,
}

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = core::config::init();
        let module = Module {
            config: config.clone(),
            rd: String::from(config.client_secret),
        };
        App::new().data(module).configure(router::init)
    })
    .bind(format!(
        "{}:{}",
        std::env::var("HOST").expect("HOST is required"),
        std::env::var("PORT").expect("PORT is required")
    ))?
    .run()
    .await
}
