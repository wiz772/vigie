use common::{Metric, MetricsPayload};
use sqlx::PgPool;

pub async fn insert_metrics(
    pool: &PgPool,
    data: &[(u64, MetricsPayload)],
    batch_id: &str,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    for (timestamp_ms, payload) in data {
        let timestamp_ms = i64::try_from(*timestamp_ms)
            .map_err(|_| sqlx::Error::Protocol("Timestamp exceeds BIGINT".into()))?;

        for sample in &payload.metrics {
            let (metric_name, value_float, value_uint) = match &sample.metric {
                Metric::CpuPercent(value) => ("CpuPercent", Some(*value), None),
                Metric::MemoryTotalBytes(value) => {
                    let value = i64::try_from(*value)
                        .map_err(|_| sqlx::Error::Protocol("Metric value exceeds BIGINT".into()))?;

                    ("MemoryTotalBytes", None, Some(value))
                }
                Metric::MemoryUsedBytes(value) => {
                    let value = i64::try_from(*value)
                        .map_err(|_| sqlx::Error::Protocol("Metric value exceeds BIGINT".into()))?;

                    ("MemoryUsedBytes", None, Some(value))
                }
                Metric::DiskTotalBytes(value) => {
                    let value = i64::try_from(*value)
                        .map_err(|_| sqlx::Error::Protocol("Metric value exceeds BIGINT".into()))?;

                    ("DiskTotalBytes", None, Some(value))
                }
                Metric::DiskUsedBytes(value) => {
                    let value = i64::try_from(*value)
                        .map_err(|_| sqlx::Error::Protocol("Metric value exceeds BIGINT".into()))?;

                    ("DiskUsedBytes", None, Some(value))
                }
            };

            sqlx::query(
                r#"
                INSERT INTO metric_samples (
                    batch_id,
                    timestamp_ms,
                    metric_type,
                    value_float,
                    value_uint,
                    component
                )
                VALUES (
                    $1,
                    $2,
                    (SELECT id FROM metric_types WHERE name = $3),
                    $4,
                    $5,
                    $6
                )
                "#,
            )
            .bind(batch_id)
            .bind(timestamp_ms)
            .bind(metric_name)
            .bind(value_float)
            .bind(value_uint)
            .bind(&sample.component)
            .execute(&mut *transaction)
            .await?;
        }
    }

    transaction.commit().await?;

    Ok(())
}
