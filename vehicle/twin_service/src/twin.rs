use crate::cloud_communicator::CloudCommunicator;
use crate::command_processor::CommandProcessor;
use crate::config::TwinServiceConfig;
use crate::vehicle_state::{VehicleCommand, VehicleState};
use crate::vehicle_state_provider::VehicleStateProvider;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use vehicle_msgs::vehicle_msgs::Vehicle;

pub struct TwinService {
    vehicle_state_provider: VehicleStateProvider,
    cloud_communicator: CloudCommunicator,
    command_processor: CommandProcessor,
    config: TwinServiceConfig,
}

impl TwinService {
    pub fn new(config: TwinServiceConfig, initial_state: Vehicle) -> Self {
        let state = Arc::new(Mutex::new(VehicleState {
            vehicle: initial_state,
            vehicle_id: config.vehicle_id.clone(),
        }));

        // TODO: properly use config to set up service's components
        let vehicle_state_provider = VehicleStateProvider::new(Arc::clone(&state));
        let cloud_communicator = CloudCommunicator::new(Arc::clone(&state));
        let command_processor = CommandProcessor::new(Arc::clone(&state));

        Self {
            vehicle_state_provider,
            cloud_communicator,
            command_processor,
            config,
        }
    }

    pub async fn run(
        &mut self,
        session: Arc<zenoh::Session>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Run the vehicle state provider to listen to vehicle signals
        let mut tasks = self.vehicle_state_provider.run(session.clone()).await?;

        let (command_tx, command_rx) = mpsc::channel::<VehicleCommand>(100);
        // Run the cloud communicator to send state and receive commands
        let mut cloud_tasks = self
            .cloud_communicator
            .run(session.clone(), command_tx)
            .await?;

        // Task to process cloud commands
        let command_processing_task = self.command_processor.run(session.clone(), command_rx);

        // Collect all tasks and await them
        tasks.append(&mut cloud_tasks);
        tasks.push(command_processing_task);

        // Await all tasks
        futures::future::join_all(tasks)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}
