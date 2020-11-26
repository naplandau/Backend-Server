use mobc::{Connection, Pool};
use mobc_redis::{redis, RedisConnectionManager};
use std::time::Duration;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = Connection<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;
#[derive(Debug)]
pub enum RedisError {
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    RedisTypeError(mobc_redis::redis::RedisError),
    RedisCMDError(mobc_redis::redis::RedisError),
    RedisClientError(mobc_redis::redis::RedisError),
}

#[derive(Clone)]
pub struct RedisFactory {
    pub pool: RedisPool,
}
impl RedisFactory {
    pub async fn connect(url: String) -> Result<Self, ()> {
        match redis::Client::open(url) {
            Ok(client) => {
                let manager = RedisConnectionManager::new(client);
                Ok(RedisFactory {
                    pool: Pool::builder()
                        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
                        .max_open(CACHE_POOL_MAX_OPEN)
                        .max_idle(CACHE_POOL_MAX_IDLE)
                        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
                        .build(manager),
                })
            }
            Err(_) => Err(()),
        }
    }
    pub async fn get_connection(&self) -> Result<RedisConnection, RedisError> {
        self.pool.get().await.map_err(|e| {
            println!("Error Connecting to Redis: {}", e);
            RedisError::RedisPoolError(e).into()
        })
    }
}
use mobc_redis::redis::{AsyncCommands, FromRedisValue};
pub async fn get_str(pool: &RedisPool, key: &str) -> Result<Option<String>, RedisError> {
    let mut con = pool.get().await.expect("msg");
    let value = con.get(key).await.expect("");
    // dbg!(value.to_owned());
    // Ok(value)
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisError::RedisTypeError(e))
}
pub async fn set_str(pool: &RedisPool, key: &str, value: &str, ttl_seconds: usize) -> Result<(),RedisError> {
    let mut con = pool.get().await.expect("msg");
    let _ :() = con.set(key, value).await.expect("");
    if ttl_seconds > 0 {
        let _ : () = con.expire(key, ttl_seconds).await.unwrap();
    }
    Ok(())
}
