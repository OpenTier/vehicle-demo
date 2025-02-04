use crate::vehicle_state::{VehicleCommand, VehicleState};
use common::DataPublisher;
use common::ZenohPublisher;
use common::ZenohSubscriber;
use log::{error, info, trace};
use prost::Message;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use vehicle_msgs::vehicle_commands::*;

const CLOUD_BATTERY_TOPIC: &str = "cloud/telemetry/battery_event";
const CLOUD_TRIP_DATA_TOPIC: &str = "cloud/telemetry/trip_data";
const CLOUD_SPEED_TOPIC: &str = "cloud/telemetry/speed";
const CLOUD_EXTERIOR_TOPIC: &str = "cloud/telemetry/exterior";
const CLOUD_TIRES_TOPIC: &str = "cloud/telemetry/tires";
const CLOUD_LOCK_STATE_TOPIC: &str = "cloud/telemetry/system_state";
const CLOUD_LOCATION_TOPIC: &str = "cloud/telemetry/location";

pub struct CloudCommunicator {
    state: Arc<Mutex<VehicleState>>,
}

impl CloudCommunicator {
    pub fn new(state: Arc<Mutex<VehicleState>>) -> Self {
        Self { state }
    }

    pub async fn run(
        &self,
        session: Arc<zenoh::Session>,
        command_tx: mpsc::Sender<VehicleCommand>,
    ) -> Result<Vec<JoinHandle<()>>, Box<dyn std::error::Error + Send + Sync>> {
        // Task to periodically publish vehicle state to the cloud
        let lock_state_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_LOCK_STATE_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_state_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing lock state event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish speed: {:?}", e);
                                }
                            } else {
                                error!("Failed to create speed event");
                            }

                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Lock State Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        let battery_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_BATTERY_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_battery_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing battery data event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish battery state: {:?}", e);
                                }
                            } else {
                                error!("Failed to create battery event");
                            }

                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Battery Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        // Task to periodically publish trip data to the cloud
        let trip_data_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_TRIP_DATA_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_trip_data_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing trip data event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish speed: {:?}", e);
                                }
                            } else {
                                error!("Failed to create speed event");
                            }

                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Trip Data Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        // Task to periodically publish speed data to the cloud
        let speed_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_SPEED_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_speed_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing speed event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish speed: {:?}", e);
                                }
                            } else {
                                error!("Failed to create speed event");
                            }

                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Speed Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        // Task to periodically publish exterior data to the cloud
        let exterior_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_EXTERIOR_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_exterior_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing exterior event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish exterior event: {:?}", e);
                                }
                            } else {
                                error!("Failed to create exterior event");
                            }
                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Exterior Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        // Task to periodically publish tires data to the cloud
        let tires_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_TIRES_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_tires_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!("Publishing tires event to the cloud: {:?}", event);
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish tires: {:?}", e);
                                }
                            } else {
                                error!("Failed to create tires event");
                            }
                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Tires Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        let location_publisher = {
            let state = Arc::clone(&self.state);
            let session = Arc::clone(&session); // Cloning session for use inside async block
            tokio::spawn(async move {
                match ZenohPublisher::new(session.clone(), CLOUD_LOCATION_TOPIC).await {
                    Ok(publisher) => {
                        loop {
                            let event = {
                                let vehicle_state = state.lock().await;
                                vehicle_state.to_current_location_event()
                            };
                            // Publish vehicle state to the cloud
                            if let Some(event) = event {
                                trace!(
                                    "Publishing current location event to the cloud: {:?}",
                                    event
                                );
                                if let Err(e) = publisher.publish(event).await {
                                    error!("Failed to publish location: {:?}", e);
                                }
                            } else {
                                error!("Failed to create location event");
                            }
                            // Wait before publishing the next state
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to create Zenoh publisher for the Location Event: {:?}",
                            e
                        );
                    }
                }
            })
        };

        // Task to receive commands from the cloud
        let command_receiver = {
            // Cloning session for use inside async block
            let session = Arc::clone(&session);
            let lock_command_subscriber =
                ZenohSubscriber::new(session.clone(), "cloud/command/VEHICLE1VIN/lock").await?;
            let turn_on_off_command_subscriber =
                ZenohSubscriber::new(session.clone(), "cloud/command/VEHICLE1VIN/turn_on_off")
                    .await?;

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        // Handle lock/unlock commands
                        lock_sample = lock_command_subscriber.subscriber.recv_async() => {
                            match lock_sample {
                                Ok(sample) => {
                                    info!("Received lock/unclock command from the cloud");
                                    let bytes = sample.payload().to_bytes();
                                    match LockUnlockCommand::decode(&*bytes) {
                                        Ok(message) => {
                                            let vehicle_command = if message.state == LockState::Lock as i32 {
                                                VehicleCommand::Lock
                                            } else {
                                                VehicleCommand::Unlock
                                            };

                                            if let Err(e) = command_tx.send(vehicle_command).await {
                                                error!("Failed to forward command: {:?}", e);
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to decode lock/unlock message: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to receive lock/unlock command: {:?}", e);
                                }
                            }
                        }

                        // Handle turn on/off commands
                        turn_on_off_sample = turn_on_off_command_subscriber.subscriber.recv_async() => {
                            match turn_on_off_sample {
                                Ok(sample) => {
                                    info!("Received turn_on_off command from the cloud");

                                    let bytes = sample.payload().to_bytes();
                                    match GeneralStateCommand::decode(&*bytes) {
                                        Ok(message) => {
                                            let vehicle_command = if message.state == CommandState::On as i32
                                                && message.target == CommandTarget::Lights as i32
                                            {
                                                VehicleCommand::LightOn
                                            } else if message.state == CommandState::Off as i32
                                                && message.target == CommandTarget::Lights as i32
                                            {
                                                VehicleCommand::LightOff
                                            } else if message.state == CommandState::On as i32
                                                && message.target == CommandTarget::Engine as i32
                                            {
                                                VehicleCommand::EngineOn
                                            } else if message.state == CommandState::Off as i32
                                                && message.target == CommandTarget::Engine as i32
                                            {
                                                VehicleCommand::EngineOff
                                            } else if message.state == CommandState::On as i32
                                                && message.target == CommandTarget::Horn as i32
                                            {
                                                VehicleCommand::HornOn
                                            } else {
                                                VehicleCommand::HornOff
                                            };

                                            if let Err(e) = command_tx.send(vehicle_command).await {
                                                error!("Failed to forward command: {:?}", e);
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to decode turn_on_off message: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to receive turn_on_off command: {:?}", e);
                                }
                            }
                        }
                    }
                }
            })
        };

        Ok(vec![
            lock_state_publisher,
            battery_publisher,
            trip_data_publisher,
            speed_publisher,
            exterior_publisher,
            tires_publisher,
            location_publisher,
            command_receiver,
        ])
    }
}
