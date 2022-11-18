// 1,接受块。
// 2，接受难度。
// 3，计算难度。
// 4，提交解决。
// 5， 不运行全节点，不连接其他网络。
// 6，把随机的nonce 改为固定的nonce
use anyhow::Result;
enum CoinBaseState {
    Start,
    Runing,
    Stop,
}

pub fn try_coinbase_solution(
    from_nonce: u64,
    to_nonce: u64,
    last_block: _,
    last_target: _,
    state: &Arc<CoinBaseState>,
) -> Result<String> {
    for nonce in from..to {
        match state {
            Start => {
                info! {"Start New Coinbase Solution"}
            }
            Runing => {
                info! {"Computing Coinbase Solution,the nonce is {}",nonce}
            }
            Stop => {
                info! {"Stopping Coinbase"}
            }
        }
    }
}
