use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub enum Metric {
    CpuPercent(f64),
    MemoryTotalBytes(u64),
    MemoryUsedBytes(u64),
    DiskTotalBytes(u64),
    DiskUsedBytes(u64),
}

#[derive(Serialize, Deserialize)]
pub struct MetricSample {
    pub metric: Metric,
    pub component: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetricsPayload {
    pub metrics: Vec<MetricSample>,
}

#[derive(Serialize, Deserialize)]
pub struct FileSystemEvent {}

#[derive(Serialize, Deserialize)]
pub struct SystemLogEvent {}

#[derive(Serialize, Deserialize)]
pub struct VigieAgentLogEvent {}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    Metrics(MetricsPayload),
    FileSystem(FileSystemEvent),
    SystemLog(SystemLogEvent),
    VigieAgentLog(VigieAgentLogEvent),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub timestamp_ms: u64,
    pub payload: Payload,
}

#[derive(Serialize, Deserialize)]
pub struct Batch {
    pub protocol_version: u32,
    pub agent_version: String,
    pub agent_id: String,
    pub host_id: String,
    pub messages: Vec<Message>,
}
