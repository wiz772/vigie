mod api;
mod config;
mod database;
mod services;
mod state;
use config::Config;
use state::AppState;

async fn start_axum(server_host: &str, server_port: u16, state: AppState) {
    let app = api::router::create_router().with_state(state);
    let listener = match tokio::net::TcpListener::bind((server_host, server_port)).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Cannot listen on {server_host}:{server_port} : {e}");
            std::process::exit(1);
        }
    };

    println!("[INFO] Listening on {}:{}...", server_host, server_port);

    match axum::serve(listener, app).await {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Cannot serve axum server : {e}");
            std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Config error: {e}");
        std::process::exit(1);
    });

    let pool = match database::connect_db(&config.database_url).await {
        Ok(pool) => {
            println!("[INFO] Connected to PostgreSQL successfully!");
            pool
        },
        Err(e) => {
            eprintln!("Database initialization error: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = database::apply_migrations(&pool).await {
        eprintln!("Database migration error: {e}");
        std::process::exit(1);
    }

    let state = AppState { db: pool };

    start_axum(&config.server_host, config.server_port, state).await;
}
