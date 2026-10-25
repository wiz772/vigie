use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub _db: PgPool,
}
