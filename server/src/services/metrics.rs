use common::MetricsPayload;
use sqlx::PgPool;

pub async fn ingest_metrics(
    pool: &PgPool,
    data: Vec<MetricsPayload>,
    batch_id: &str,
    host_id: &str,
) {
}
