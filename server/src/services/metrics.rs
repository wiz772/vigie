use common::MetricsPayload;
use sqlx::PgPool;

use crate::repositories;

#[allow(clippy::enum_variant_names)]
enum MetricValidationError {
    _InvalidCpuUsage(f64),
    _InvalidMemoryUsage,
    _InvalidDiskUsage,
}

pub async fn ingest_metrics(
    pool: &PgPool,
    data: Vec<(u64, MetricsPayload)>,
    batch_id: &str,
) -> Result<(), sqlx::Error> {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for (timestamp_ms, payload) in data {
        match validate_metrics(payload) {
            Ok(payload) => valid.push((timestamp_ms, payload)),
            Err(error) => invalid.push((timestamp_ms, error)),
        }
    }

    // ingest les correct
    repositories::metrics::insert_metrics(pool, &valid, batch_id).await?;

    Ok(())
    // later  ingest les errreurs sur vigielog
}

fn validate_metrics(payload: MetricsPayload) -> Result<MetricsPayload, MetricValidationError> {
    Ok(payload)
}
