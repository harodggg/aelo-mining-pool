mod prover;
pub mod rpc;
mod trace;
pub mod version;
use lazy_static::lazy_static;
use std::str;
use std::sync::Mutex;

// use stratum::AleoStratum;
// Create a global mutable work queue.
lazy_static! {
    pub static ref WORK_QUEUE: Mutex<Vec<String>> =
        Mutex::new(vec!["work1".to_owned(), "worker2".to_owned()]);
}

#[derive(Default)]
struct PoolClient<'a> {
    name: &'a str,
    //todo! change AleoStratum to AleoStratumWorker
    // stratum_client: AleoStratum,
    version: &'a str,
}
impl PoolClient<'_> {
    pub fn start_up() {
        //todo! 启动客户端
        unimplemented!()
        //todo! 开始循环，启动多线程，订阅service
        //todo! 1,接受来自service的job
        //todo! 2,多线程执行job,如果计算出正确的solution，就提交正确结果。
        //todo! 3,当新的任务被刷时，立刻，停止所有的多线程，并执行新的job。
    }
}
