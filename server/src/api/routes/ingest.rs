use crate::services::metrics;
use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::post};
use common::{Batch, Payload};

async fn ingest_batch(State(state): State<AppState>, Json(batch): Json<Batch>) {
    let mut metrics_batch = Vec::new();

    for message in batch.messages {
        match message.payload {
            Payload::Metrics(data) => {
                metrics_batch.push(data);
            }
            Payload::FileSystem(_data) => {}
            Payload::SystemLog(_data) => {}
            Payload::VigieAgentLog(_data) => {}
        }
    }

    metrics::ingest_metrics(&state.db, metrics_batch, &batch.batch_id,&batch.agent_id, batch.protocol_version,).await;
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/ingest", post(ingest_batch))
}
