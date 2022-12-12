use anyhow::Result;
use stratum_pool::stratum_pool::{
    stratum_pool_client::StratumPoolClient, AuthorizeRequest, SubscribeRequest,
};
use tokio::time::Duration;
use tonic;

const POOL_CONNECT: &str = "http://[::1]:50051";

// auth login
// worker -> rpc service -> prover -> worker -> status
pub async fn rpc_client_run() -> Result<()> {
    loop {
        let mut client = StratumPoolClient::connect(POOL_CONNECT).await?;
        let request = tonic::Request::new(AuthorizeRequest { id: 1 });
        let response = client.mining_authorize(request).await?;
        println!("RESPONSE={:#?}", response.get_ref());
        worker_subscribe(
            "worker 1".to_string(),
            "0.0.1".to_string(),
            "127.0.0.1:5000".to_string(),
        )
        .await?;
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

// submit block
pub async fn submit_block() -> Result<bool> {
    Ok(true)
}

pub async fn worker_auth() {}

pub async fn worker_nofity() {}

// worker subscribe for pool by grpc
// return worker subscribe status
pub async fn worker_subscribe(
    worker_name: String,
    stratum_version: String,
    worker_rpc_server: String,
) -> Result<()> {
    let mut client = StratumPoolClient::connect(POOL_CONNECT)
        .await
        .expect("Pool Connnect Error");
    let request = tonic::Request::new(SubscribeRequest {
        worker_name: worker_name,
        stratum_version: stratum_version,
        worker_rpc_server: worker_rpc_server,
    });
    let response = client.mining_subscribe(request).await?;
    println!("RESPONSE={:#?}", response.get_ref());
    Ok(())
}
