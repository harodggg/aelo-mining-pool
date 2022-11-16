mod rpc;

use rpc::block;
use stratum;

async fn run_rpc_service() {
    stratum::add(1 + 1);
}
