use anyhow::Result;
use snarkvm::prelude::Address;
use std::net::SocketAddr;

// redis
use redis::Client;
use redis::Commands;
use snarkvm::prelude::Network;

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
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
    redis: Client,
}

impl DB {
    pub fn connect_db() -> Result<bool> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(true)
    }

    pub fn store_worker() -> Result<bool> {
        unimplemented!();
    }
    pub fn delete_worker() -> Result<bool> {
        unimplemented!()
    }
    pub fn get_worker() -> Result<bool> {
        unimplemented!()
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
