// This code was developed by OpenTier GmbH.
use async_trait::async_trait;
use prost::Message;
use std::sync::Arc;
use zenoh::pubsub::Publisher;
use zenoh::Session;

#[async_trait]
pub trait DataPublisher<'a, T>
where
    T: Message + Send + Sync + 'static,
{
    async fn publish(&self, data: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct ZenohPublisher<'a> {
    publisher: Publisher<'a>,
}

impl<'a> ZenohPublisher<'a> {
    pub async fn new(
        session: Arc<Session>,
        key_expr: &'static str,
    ) -> Result<ZenohPublisher<'a>, Box<dyn std::error::Error + Send + Sync>> {
        let publisher = session.declare_publisher(key_expr).await?;
        Ok(ZenohPublisher { publisher })
    }
}

#[async_trait]
impl<'a, T> DataPublisher<'a, T> for ZenohPublisher<'a>
where
    T: Message + Send + Sync + 'static,
{
    async fn publish(&self, data: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = data.encode_to_vec();
        self.publisher.put(payload).await
    }
}
