use common::Batch;

#[test]
fn batch_can_be_serialized() {
    let batch = Batch {
        protocol_version: common::PROTOCOL_VERSION,
        agent_version: "0.1.0".to_string(),
        agent_id: "agent-001-test".to_string(),
        host_id: "host-001-test".to_string(),
        messages: vec![],
    };

    let json = serde_json::to_string_pretty(&batch).unwrap();

    println!("{json}");
}
