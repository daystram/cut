use crate::core;
use crate::AppConfig;
use actix_web::{middleware::Logger, App, HttpServer};

pub mod controllers;
pub mod handlers;
pub mod router;

pub struct Module {
    pub config: AppConfig,
}

pub async fn start() -> std::io::Result<()> {
    // init AppConfig
    let config = core::config::init();

    // run server
    HttpServer::new(move || {
        App::new()
            .data(Module {
                config: config.clone(),
            })
            .wrap(Logger::default())
            .configure(router::init)
    })
    .bind(format!(
        "{}:{}",
        std::env::var("HOST").expect("HOST is required"),
        std::env::var("PORT").expect("PORT is required")
    ))?
    .run()
    .await
}
