use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RootConfig {
    pub signal_mocker_service: SignalMockerServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct SignalMockerServiceConfig {
    pub messages: HashMap<String, MessageConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MessageConfig {
    pub frequency: u64,
    pub signals: HashMap<String, SignalOrNestedMessage>,
}

// This enum allows a signal to either be a regular signal or a nested message
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SignalOrNestedMessage {
    Signal(SignalConfig),
    NestedMessage(HashMap<String, SignalOrNestedMessage>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Static,
    Interpolated,
    Timestamp,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SignalConfig {
    pub data_type: DataType,
    pub data: Option<Vec<f64>>, // For numeric data (static/interpolated)
    pub data_string: Option<Vec<String>>, // For static string data
    pub data_bool: Option<Vec<bool>>, // For static bool data
    pub start_value: Option<f64>, // For interpolated data
    pub end_value: Option<f64>, // For interpolated data
    pub steps: Option<u64>,     // For interpolated data
    pub noise_level: Option<f64>, // For interpolated data
}

pub fn extract_signals(
    input: &HashMap<String, SignalOrNestedMessage>,
) -> HashMap<String, SignalConfig> {
    let mut result = HashMap::new();

    for (key, value) in input {
        match value {
            SignalOrNestedMessage::Signal(signal) => {
                result.insert(key.clone(), signal.clone());
            }
            SignalOrNestedMessage::NestedMessage(_) => {
                // Handle nested messages if needed, or skip them
            }
        }
    }

    result
}
