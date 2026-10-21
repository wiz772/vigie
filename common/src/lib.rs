pub const PROTOCOL_VERSION: u32 = 1;


pub enum MetricType {
    CpuPercent,
    MemoryTotalBytes,
    MemoryUsedBytes,
    DiskTotalBytes,
    DiskUsedBytes,
}

pub enum MetricValue {
    Decimal(f64),
    Integer(u64),
}

pub struct Metrics {
    pub metric_type: MetricType,
    pub metric_value: MetricValue,
    pub component: String
}

pub struct MetricLogs{
    pub metrics: Vec<Metrics>
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
    MetricLogs(MetricLogs),
    FileSystem(FileSystemEvent),
    SystemLog(SystemLogEvent),
    VigieAgentLog(VigieAgentLogEvent)
}
