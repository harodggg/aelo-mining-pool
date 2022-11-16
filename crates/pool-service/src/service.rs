mod rpc;

use rpc::block;

async fn run_rpc_service() {
    stratum::add(1 + 1);
}
