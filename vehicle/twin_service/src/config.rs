use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwinServiceConfig {
    pub zenoh_endpoints: Vec<String>,
    pub vehicle_id: String,
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub topic: String,
    pub frequency: u64, // in milliseconds
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub topic: String,
}
