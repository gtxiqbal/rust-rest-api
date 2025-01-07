mod configs;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod router;
mod services;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = configs::app::App::init().await?;
    app.run().await
}