use crate::core::config::AppConfig;

mod app;
mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").ok().is_none() {
        std::env::set_var("RUST_LOG", "debug,actix_web=info");
    }
    env_logger::init();

    app::start().await
}
