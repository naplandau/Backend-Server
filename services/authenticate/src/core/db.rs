use crate::config::CONFIG;
use crate::core::redis_db::*;
use crate::nats_broker::*;
use mongodb::Client;
use once_cell::sync::OnceCell;

static MONGO: OnceCell<Client> = OnceCell::new();
static MONGO_INIT: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

static NATS: OnceCell<NatsConnection> = OnceCell::new();
static NATS_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

static REDIS: OnceCell<RedisPool> = OnceCell::new();
static REDIS_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_mongo() -> Option<&'static Client> {
    if let Some(v) = MONGO.get() {
        return Some(v);
    }

    let initializing_mutex = MONGO_INIT.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        let database_url = &CONFIG.database_url;
        if let Ok(client) = Client::with_uri_str(database_url.as_str()).await {
            if let Ok(_) = MONGO.set(client) {
                *initialized = true;
            }
        }
    }
    drop(initialized);
    MONGO.get()
}
pub async fn get_nats() -> Option<&'static NatsConnection> {
    let client_option = NATS.get();
    if let Some(_) = client_option {
        return client_option;
    }
    let initializing_mutex = NATS_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;
    if !*initialized {
        let database_url = &CONFIG.nats_url;
        if let Ok(client) = NatsFactory::get_pool(database_url.to_owned()).await {
            if let Ok(_) = NATS.set(client) {
                *initialized = true;
            }
        }
    }
    drop(initialized);
    NATS.get()
}
pub async fn get_redis() -> Option<&'static RedisPool> {
    let client_option = REDIS.get();
    if let Some(_) = client_option {
        return client_option;
    }
    let initializing_mutex = NATS_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;
    if !*initialized {
        let database_url = &CONFIG.redis_url;
        if let Ok(client) = RedisFactory::connect(database_url.to_owned()).await {
            if let Ok(_) = REDIS.set(client) {
                *initialized = true;
            }
        }
    }
    drop(initialized);
    REDIS.get()
}
