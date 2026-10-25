use crate::services::metrics;
use crate::state::AppState;
use axum::{Json, Router, routing::post};
use common::{Batch, Payload};

async fn ingest_batch(Json(batch): Json<Batch>) {
    // let mut total_metrics = 0;
    // let mut total_filesystem = 0;
    // let mut total_systemlog = 0;
    // let mut total_vigieagentlog = 0;

    for message in batch.messages {
        match message.payload {
            Payload::Metrics(data) => {
                metrics::ingest_metrics(data, &batch.host_id);
                // total_metrics += 1;
            }
            Payload::FileSystem(_data) => {
                // total_filesystem += 1;
            }
            Payload::SystemLog(_data) => {
                // total_systemlog += 1;
            }
            Payload::VigieAgentLog(_data) => {
                // total_vigieagentlog += 1;
            }
        }
    }

    // send to serverlogger
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/ingest", post(ingest_batch))
}
