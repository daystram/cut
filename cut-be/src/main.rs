mod app;
mod core;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").ok().is_none() {
        std::env::set_var("RUST_LOG", "info,actix_web=info");
    }
    env_logger::init();

    app::start().await
}
