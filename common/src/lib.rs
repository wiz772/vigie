pub const PROTOCOL_VERSION: u32 = 1;

pub struct Metric{

}

pub struct FileSystemEvent{

}

pub struct LogEvent{

}

pub struct Batch{
    pub protocol_version: u32,
    pub agent_version: String,
    pub agent_id: String,
    pub content: Vec<Message>,

}

pub struct Message{
    pub timestamp: u64,
    pub payload: Payload
}

pub enum Payload {
    Metrics(Metric),
    FileSystem(FileSystemEvent),
    Log(LogEvent),
}

