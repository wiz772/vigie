use axum::{Router, routing::get};

async fn health() -> &'static str {
    "ok"
}

pub fn router() -> Router {
    Router::new().route("/api/v1/health", get(health))
}
