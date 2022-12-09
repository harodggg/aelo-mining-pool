use anyhow::Result;
use mobc_redis::redis::aio::Connection;
use snarkvm::prelude::Address;
use std::net::SocketAddr;
use std::time::Duration;

// redis
use mobc::Pool;
use mobc_redis::redis::RedisError;
use mobc_redis::redis::{self, FromRedisValue};
use mobc_redis::RedisConnectionManager;
use snarkvm::prelude::Network;

pub type MobcCon = Connection<RedisConnectionManager>;
pub type MobcPool = Pool<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;
const REDIS_CON_STRING: &str = "redis://127.0.0.1:6379";

pub async fn connect() -> Result<MobcPool> {
    let client = redis::Client::open(REDIS_CON_STRING)?;
    let manager = RedisConnectionManager::new(client);
    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager))
}

async fn get_con(pool: &MobcPool) -> Result<MobcCon> {
    unimplemented!()
}

pub async fn set_str(pool: &MobcPool, key: &str, value: &str, ttl_seconds: usize) -> Result<()> {
    let mut con = get_con(&pool).await?;

    Ok(())
}

pub async fn get_str(pool: &MobcPool, key: &str) -> Result<String> {
    let mut con = get_con(&pool).await?;
    //FromRedisValue::from_redis_value(&value).map_err(|e| Error(e).into())
    Ok(String::from("rrl"))
}

pub fn add_woker() -> Result<String, ()> {
    Ok("harold".to_string())
}

pub fn get_block() {
    unimplemented!()
}

pub fn get_blockhead() {
    unimplemented!()
}

pub fn get_worker() {
    unimplemented!()
}

struct DB {
    // The redis connect pool
    redis_pool: Pool<RedisConnectionManager>,
}

impl DB {
    pub fn connect_db(&mut self) -> Result<bool> {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder().max_open(20).build(manager);
        self.redis_pool = pool;
        Ok(true)
    }

    pub fn store_worker(&mut self, worker_name: &str, work_content: &str) -> Result<bool> {
        Ok(true)
    }
    pub fn delete_worker(&mut self, worker_name: &str) -> Result<bool> {
        Ok(true)
    }
    pub fn get_worker(&mut self, worker_name: &str) -> Result<bool> {
        Ok(true)
    }
    pub fn save_data(&mut self, filename: &str) -> Result<bool> {
        Ok(true)
    }
}

enum WorkerStatus {
    Registered,
    Noregister,
    Runing,
    Quited,
}

struct Worker<'a, N: Network> {
    status: WorkerStatus,
    name: &'a str,
    ip: SocketAddr,
    password: &'a str,
    aleo_address: Address<N>,
}
