pub mod log;
mod runtime;
mod version;
pub use version::*;

// 1，设计挖矿速度的单位 pr/s 每一秒证明生成时间。 挖矿效率：Pr/s。
// 2，计算大约15s。根据挖矿的速度的单位，分配任务。
// 3,
use anyhow::Result;
pub fn computing_performance(//需要被测试的块-需要的测试的结构。
) -> Result<u64> {
    let start_time: u64;
    let end_time: u64;
    // why not?
    Ok(33)
}
