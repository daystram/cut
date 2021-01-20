use crate::core::config;
use actix_web::{middleware::Logger, App, HttpServer};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub mod constants;
pub mod controllers;
pub mod datatransfers;
pub mod handlers;
pub mod router;

pub struct Module {
    pub config: config::AppConfig,
    pub rd_pool: r2d2::Pool<RedisConnectionManager>,
}

pub async fn start() -> std::io::Result<()> {
    // init AppConfig
    let config = config::init();

    // init Redis
    let rd_manager = RedisConnectionManager::new(format!(
        "redis://{}:{}/",
        config.redis_host, config.redis_port
    ))
    .unwrap();
    let rd_pool = Pool::builder().max_size(15).build(rd_manager).unwrap();
    log::info!("[INIT] Redis initialized!");

    // run server
    HttpServer::new(move || {
        App::new()
            .data(Module {
                config: config.clone(),
                rd_pool: rd_pool.clone(),
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
