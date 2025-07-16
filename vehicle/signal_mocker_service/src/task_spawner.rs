// This code was developed by OpenTier GmbH.
use crate::generators::MessageGenerator;
use common::publishers::{DataPublisher, ZenohPublisher};
use log::error;
use prost::Message;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use zenoh::session::Session;

pub struct PublicationTaskSpawner;

impl PublicationTaskSpawner {
    // Static method to spawn the task
    pub fn spawn_task<G, T>(
        session: Arc<Session>,
        key_expr: &'static str,
        mut generator: G,
        frequency: Duration,
    ) -> JoinHandle<()>
    where
        G: MessageGenerator<T> + Send + 'static,
        T: Message + Send + Sync + 'static,
    {
        tokio::spawn(async move {
            // Create ZenohPublisher asynchronously
            match ZenohPublisher::new(session, key_expr).await {
                Ok(publisher) => {
                    loop {
                        // Generate the message
                        let msg = generator.generate();

                        // Publish the generated message
                        if let Err(e) = publisher.publish(msg).await {
                            error!("Failed to publish: {:?}", e);
                        }

                        // Sleep for the scheduled frequency
                        sleep(frequency).await;
                    }
                }
                Err(e) => {
                    error!("Failed to create ZenohPublisher: {:?}", e);
                }
            }
        })
    }
}
