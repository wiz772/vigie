pub const PROTOCOL_VERSION: u32 = 1;

pub struct Metric{}

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
    Metric(Metric),
    FileSystem(FileSystemEvent),
    SystemLog(SystemLogEvent),
    VigieAgentLog(VigieAgentLogEvent)
}
