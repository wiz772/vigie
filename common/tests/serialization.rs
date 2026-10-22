use common::{Batch, Message, Metric, MetricSample, MetricsPayload, Payload};

#[test]
fn batch_can_be_serialized_and_deserialized() {
    let batch = Batch {
        protocol_version: common::PROTOCOL_VERSION,
        agent_version: "0.1.0".to_string(),
        agent_id: "agent-001-test".to_string(),
        host_id: "host-001-test".to_string(),
        batch_id: "a1zczq2E2E2d".to_string(),
        messages: vec![Message {
            timestamp_ms: 1792600000000,
            payload: Payload::Metrics(MetricsPayload {
                metrics: vec![MetricSample {
                    metric: Metric::CpuPercent(42.5),
                    component: "cpu".to_string(),
                }],
            }),
        }],
    };

    // 1. Batch -> JSON
    let json = serde_json::to_string_pretty(&batch).unwrap();
    println!("{json}");

    // 2. JSON -> Batch
    let decoded: Batch = serde_json::from_str(&json).unwrap();

    let json_again = serde_json::to_string_pretty(&decoded).unwrap();
    assert_eq!(json, json_again);
}
