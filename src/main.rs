mod config;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod services;
mod utils;
mod router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = config::app::App::init().await?;
    app.run().await
}