use crate::repositories::batch::insert_batch;
use crate::services::metrics::ingest_metrics;
use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::post};
use common::{Batch, Payload};

async fn ingest_batch(State(state): State<AppState>, Json(batch): Json<Batch>) {
    let mut metrics_batch = Vec::new();

    for message in batch.messages {
        match message.payload {
            Payload::Metrics(data) => {
                metrics_batch.push((message.timestamp_ms, data));
            }
            Payload::FileSystem(_data) => {}
            Payload::SystemLog(_data) => {}
            Payload::VigieAgentLog(_data) => {}
        }
    }

    let batch_result = insert_batch(
        &state.db,
        &batch.batch_id,
        &batch.agent_id,
        batch.protocol_version,
    )
    .await;
    match batch_result {
        Ok(()) => {
            let metric_result = ingest_metrics(&state.db, metrics_batch, &batch.batch_id).await;
            match metric_result {
                Ok(()) => println!("Succeeded to ingest METRICS"),
                Err(error) => println!("Failure to ingest METRICS: {}", error),
            }
        }
        Err(error) => println!("Failure to INSERT BATCH: {}", error),
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/ingest", post(ingest_batch))
}
