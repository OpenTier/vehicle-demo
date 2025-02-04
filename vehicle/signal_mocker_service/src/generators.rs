// This code was developed by OPENTIER - FZCO.
use crate::config::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SignalState {
    current_index: usize, // For iterating over static arrays
    current_step: f64,    // For interpolated values
}

pub trait MessageGenerator<T>
where
    T: prost::Message + Send + Sync + 'static,
{
    fn generate(&mut self) -> T;
}

pub struct SignalGenerator {
    config: HashMap<String, SignalConfig>, // Store the configuration for signals
    signal_state: HashMap<String, SignalState>, // Store the state of each signal
}

impl SignalGenerator {
    pub fn new(config: HashMap<String, SignalConfig>) -> Self {
        let mut signal_state = HashMap::new();

        // Initialize state for each signal
        for (key, signal) in &config {
            let initial_state = SignalState {
                current_index: 0,
                current_step: signal.start_value.unwrap_or(0.0), // For interpolated values
            };
            signal_state.insert(key.clone(), initial_state);
        }

        Self {
            config,
            signal_state,
        }
    }

    // Function to get the next value of a signal
    pub fn get_next_signal_value(&mut self, signal_name: &str) -> Option<f64> {
        if let Some(signal) = self.config.get(signal_name) {
            let state = self.signal_state.get_mut(signal_name)?;

            match signal.data_type {
                DataType::Static => {
                    if let Some(data) = &signal.data {
                        // Return the current value and advance the index
                        let value = data.get(state.current_index).copied();
                        state.current_index = (state.current_index + 1) % data.len(); // Cycle through static array
                        return value;
                    }
                }
                DataType::Interpolated => {
                    if let (Some(start), Some(end), Some(steps)) =
                        (signal.start_value, signal.end_value, signal.steps)
                    {
                        // Calculate the step value based on whether the end value is greater or less than the start value
                        let mut step_value = if end > start {
                            (end - start) / (steps as f64)
                        } else {
                            (start - end) / (steps as f64)
                        };

                        // Add noise if noise_level is specified
                        if let Some(noise_level) = signal.noise_level {
                            let mut rng = rand::thread_rng();
                            let noise: f64 = rng.gen_range(0.0..=noise_level); // Generate random noise between 0 and noise_level (inclusive)
                            step_value += noise;
                        }

                        let value = Some(state.current_step);

                        // Move to the next step
                        if end > start {
                            state.current_step += step_value;
                            if state.current_step > end {
                                state.current_step = start; // Loop back to the start if we go beyond the end
                            }
                        } else {
                            state.current_step -= step_value;
                            if state.current_step < end {
                                state.current_step = start; // Loop back to the start if we go below the end
                            }
                        }

                        return value;
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn get_next_signal_string(&mut self, signal_name: &str) -> Option<String> {
        if let Some(signal) = self.config.get(signal_name) {
            if let Some(data) = &signal.data_string {
                let state = self.signal_state.get_mut(signal_name)?;
                let value = data.get(state.current_index).cloned();
                state.current_index = (state.current_index + 1) % data.len(); // Cycle through the string data
                value
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_next_signal_bool(&mut self, signal_name: &str) -> Option<bool> {
        if let Some(signal) = self.config.get(signal_name) {
            let state = self.signal_state.get_mut(signal_name)?;

            if let Some(data) = &signal.data_bool {
                let value = data.get(state.current_index).copied();
                state.current_index = (state.current_index + 1) % data.len();
                return value;
            }
        }
        None
    }
}

#[macro_export]
macro_rules! define_generator {
    ($name:ident $(, $field_name:ident : $field_type:ty)*) => {
        pub struct $name {
            pub signal_generator: SignalGenerator,
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub fn new(config: HashMap<String, SignalConfig> $(, $field_name: $field_type)*) -> Self {
                Self {
                    signal_generator: SignalGenerator::new(config),
                    $($field_name,)*
                }
            }

            pub fn get_next_signal_value(&mut self, signal_name: &str) -> Option<f64> {
                self.signal_generator.get_next_signal_value(signal_name)
            }

            pub fn get_next_signal_string(&mut self, signal_name: &str) -> Option<String> {
                self.signal_generator.get_next_signal_string(signal_name)
            }
        }
    };
}
