
pub const PROTOCOL_VERSION: u32 = 1;

pub enum Metric {
    CpuPercent(f64),
    MemoryTotalBytes(u64),
    MemoryUsedBytes(u64),
    DiskTotalBytes(u64),
    DiskUsedBytes(u64),
}

pub struct MetricSample {
    pub metric: Metric,
    pub component: String,
}

pub struct MetricsPayload {
    pub metrics: Vec<MetricSample>,
}

pub struct FileSystemEvent {}

pub struct SystemLogEvent {}

pub struct VigieAgentLogEvent {}

pub struct Batch {
    pub protocol_version: u32,
    pub agent_version: String,
    pub agent_id: String,
    pub host_id: String,
    pub messages: Vec<Message>,
}

pub struct Message {
    pub timestamp_ms: u64,
    pub payload: Payload,
}

pub enum Payload {
    Metrics(MetricsPayload),
    FileSystem(FileSystemEvent),
    SystemLog(SystemLogEvent),
    VigieAgentLog(VigieAgentLogEvent),
}
