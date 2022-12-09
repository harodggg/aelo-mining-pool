pub mod block_observer;
mod db;
pub mod rpc;
pub mod version;
pub mod block {
    tonic::include_proto!("block");
}

#[derive(Default)]
struct PoolService {
    name: String,
    version: String,
    // todo!,1,启动stratum rpc 任务分发协议。
    // todo!,2，获取最新的块，最新的参数。
    // todo!,3, 接受client的注册。
    // todo!,4, 根据服务器的数据，不管的新的任务块。
    // todo!,5, 接受solution。
    // todo!,5, 处理日志问题。将snakos的日志库，给禁止掉。不输出不必要的信息。
}

impl PoolService {
    pub fn start_up(&self) {}
}

trait Rpcable {}
