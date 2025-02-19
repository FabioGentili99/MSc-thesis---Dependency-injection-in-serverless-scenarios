use async_nats::Client;
use async_trait::async_trait;
use bb8::{ManageConnection, Pool, PooledConnection};
use std::time::Duration;

pub struct NatsManager {
    addr: String,
}

impl NatsManager {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }
}

#[async_trait]
impl ManageConnection for NatsManager {
    type Connection = Client;
    type Error = async_nats::ConnectError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        async_nats::connect(&self.addr).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        if conn.flush().await.is_ok() {
            Ok(())
        } else {
            Err(async_nats::ConnectError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Connection is broken",
            )))
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

pub type NatsPool = Pool<NatsManager>;

pub async fn create_nats_pool(nats_url: &str, max_size: u32) -> NatsPool {
    let manager = NatsManager::new(nats_url);
    Pool::builder()
        .max_size(max_size)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .await
        .expect("Failed to create NATS pool")
}
