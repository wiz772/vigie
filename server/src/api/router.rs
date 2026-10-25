use super::routes;
use crate::state::AppState;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(routes::health::router())
        .merge(routes::ingest::router())
}
