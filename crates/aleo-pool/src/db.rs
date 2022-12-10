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

struct DB {
    // The redis connect pool
    redis_pool: Pool<RedisConnectionManager>,
}

impl DB {
    pub async fn connect_db(&mut self) {
        let client = redis::Client::open(REDIS_CON_STRING).expect("redis connect error");
        let manager = RedisConnectionManager::new(client);
        self.redis_pool = Pool::builder()
            .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .build(manager);
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
