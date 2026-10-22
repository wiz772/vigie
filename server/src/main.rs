use tokio;
mod config;

async fn start_axum() {}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // connect db
    start_axum().await;
}
