use common::{Metric, MetricSample, MetricsPayload};
use server::services::metrics::ingest_metrics;
use sqlx::PgPool;

#[sqlx::test(migrations = "./migrations")]
async fn test_metric_ingestion(pool: PgPool) {
    let batch_id = "test-batch-001";

    let payload = MetricsPayload {
        metrics: vec![
            MetricSample {
                metric: Metric::CpuPercent(45.5),
                component: "cpu".to_string(),
            },
            MetricSample {
                metric: Metric::MemoryTotalBytes(16_000_000_000),
                component: "memory".to_string(),
            },
            MetricSample {
                metric: Metric::MemoryUsedBytes(8_000_000_000),
                component: "memory".to_string(),
            },
            MetricSample {
                metric: Metric::DiskTotalBytes(500_000_000_000),
                component: "disk".to_string(),
            },
            MetricSample {
                metric: Metric::DiskUsedBytes(250_000_000_000),
                component: "disk".to_string(),
            },
        ],
    };

    let data = vec![(1793500000000_u64, payload)];

    sqlx::query("INSERT INTO hosts (host_id, hostname) VALUES ($1, $2)")
        .bind("test-host-001")
        .bind("test-machine")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO agents (agent_id, host_id, agent_version) VALUES ($1, $2, $3)")
        .bind("test-agent-001")
        .bind("test-host-001")
        .bind("0.1.0")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO batches (batch_id, agent_id, protocol_version) VALUES ($1, $2, $3)")
        .bind(batch_id)
        .bind("test-agent-001")
        .bind(1_i32)
        .execute(&pool)
        .await
        .unwrap();

    ingest_metrics(&pool, data, batch_id)
        .await
        .expect("Metrics ingestion should succeed");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM metric_samples WHERE batch_id = $1")
        .bind(batch_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count, 5);

    let timestamps: Vec<i64> =
        sqlx::query_scalar("SELECT DISTINCT timestamp_ms FROM metric_samples WHERE batch_id = $1")
            .bind(batch_id)
            .fetch_all(&pool)
            .await
            .unwrap();

    assert_eq!(timestamps, vec![1793500000000_i64]);
}
