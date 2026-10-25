use crate::state::AppState;
use axum::{Router, routing::get};

async fn health() -> &'static str {
    "ok"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/health", get(health))
}
