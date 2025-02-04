use crate::vehicle_state::{VehicleCommand, VehicleState};
use common::topics::LOCK_STATE_TOPIC;
use common::DataPublisher;
use common::ZenohPublisher;
use log::{error, info, warn};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct CommandProcessor {
    state: Arc<Mutex<VehicleState>>, // VehicleState to verify commands
}

impl CommandProcessor {
    pub fn new(state: Arc<Mutex<VehicleState>>) -> Self {
        Self { state }
    }

    pub fn run(
        &self,
        session: Arc<zenoh::Session>,
        mut command_rx: mpsc::Receiver<VehicleCommand>,
    ) -> JoinHandle<()> {
        let vehicle_state = self.state.clone();
        tokio::spawn(async move {
            match ZenohPublisher::new(session, LOCK_STATE_TOPIC).await {
                Ok(state_publisher) => {
                    while let Some(command) = command_rx.recv().await {
                        info!("Received command from cloud: {:?}", command);

                        // Validate the command
                        {
                            if !vehicle_state.lock().await.is_valid_command(&command) {
                                warn!("Invalid command: {:?}", command);
                                continue;
                            }
                        }

                        // Forward the command to the in-vehicle system
                        match command {
                            VehicleCommand::Lock => {
                                info!("Forwarding Lock command to in-vehicle system");
                                let new_state = vehicle_msgs::state::LockState {
                                    state: vehicle_msgs::state::State::Lock as i32,
                                };
                                match state_publisher.publish(new_state).await {
                                    Ok(_) => {
                                        info!("Published lock state");
                                    }
                                    Err(e) => {
                                        error!("Failed to publish lock state {:?}", e);
                                    }
                                }
                            }
                            VehicleCommand::Unlock => {
                                info!("Forwarding Unlock command to in-vehicle system");
                                let new_state = vehicle_msgs::state::LockState {
                                    state: vehicle_msgs::state::State::On as i32,
                                };

                                // Publish the new unlock state
                                match state_publisher.publish(new_state).await {
                                    Ok(_) => {
                                        info!("Published unlock state");
                                    }
                                    Err(e) => {
                                        error!("Failed to publish unlock state: {:?}", e);
                                    }
                                }
                            }
                            VehicleCommand::LightOn => {
                                info!("Forwarding LightOn command to in-vehicle system");
                                // command_publisher.publish(command).await?;
                            }
                            VehicleCommand::LightOff => {
                                info!("Forwarding LightOff command to in-vehicle system");
                                // command_publisher.publish(command).await?;
                            }
                            VehicleCommand::HornOn => {
                                info!("Forwarding HornOn command to in-vehicle system");
                                // command_publisher.publish(command).await?;
                            }
                            VehicleCommand::HornOff => {
                                info!("Forwarding HornOff command to in-vehicle system");
                                // command_publisher.publish(command).await?;
                            }
                            VehicleCommand::EngineOn => {
                                info!("Forwarding Unlock command to in-vehicle system");
                                // command_publisher.EngineOn(command).await?;
                            }
                            VehicleCommand::EngineOff => {
                                info!("Forwarding EngineOff command to in-vehicle system");
                                // command_publisher.publish(command).await?;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to create Zenoh publisher for vehicle commands: {:?}",
                        e
                    );
                }
            }
        })
    }
}
