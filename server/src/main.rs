mod config;
use axum::{Router, routing::get};
use config::Config;

async fn start_axum(server_host: &str, server_port: u16) {
    let app: Router = Router::new().route("/health", get(|| async { "ok" }));
    let listener = match tokio::net::TcpListener::bind((server_host, server_port)).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Cannot listen on {server_host}:{server_port} : {e}");
            std::process::exit(1);
        }
    };

    println!("Listening on {}:{}...", server_host, server_port);

    match axum::serve(listener, app).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Cannot serve axum server : {e}");
            std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config error : {e}");
            std::process::exit(1);
        }
    };

    // connect db
    // start axum
    start_axum(&config.server_host, config.server_port).await;
}
