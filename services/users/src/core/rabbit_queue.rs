use mobc::{Pool, Connection};
use mobc_lapin::RMQConnectionManager;
// use tokio_amqp::*;
// use futures::StreamExt;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties,
    ConnectionProperties, Consumer,
};
use std::time::Duration;

pub type RabbitPool = Pool<RMQConnectionManager>;
pub type RabbitConnection = Connection<RMQConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;
#[derive(Debug)]
pub enum RabbitError {
    RabbitPoolError(mobc::Error<mobc_lapin::lapin::Error>),
    RabbitTypeError(mobc_lapin::lapin::Error),
    RabbitCMDError(mobc_lapin::lapin::Error),
    RabbitClientError(mobc_lapin::lapin::Error),
}

#[derive(Clone)]
pub struct RabbitFactory;
impl RabbitFactory {
    pub async fn get_pool(url: String) -> Result<RabbitPool, ()> {
        let manager = RMQConnectionManager::new(url.to_owned(), ConnectionProperties::default());
        Ok(Pool::builder()
            .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .build(manager))
    }
}
pub struct RabbitServer;
impl RabbitServer {
    pub async fn create_consumer(
        conn: &RabbitConnection,
        queue_name: &str,
        consumer_tag: &str,
    ) -> Result<Consumer, ()> {
        let channel = conn.create_channel().await.unwrap();
        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        println!("Decleared queue {:#?}", queue);
        let consumer_open = channel
            .basic_consume(
                queue_name,
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await;
        match consumer_open {
            Ok(consumer) => Ok(consumer),
            Err(_e) => Err(()),
        }
    }
}
