use super::routes;
use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .merge(routes::health::router())
}
