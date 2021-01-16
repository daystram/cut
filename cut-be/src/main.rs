use crate::core::config::AppConfig;

mod app;
mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    app::start().await
}
