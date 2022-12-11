use anyhow::Result;
use stratum_pool::stratum_pool::{stratum_pool_client::StratumPoolClient, AuthorizeRequest};
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
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

// submit block
pub async fn submit_block() -> Result<bool> {
    Ok(true)
}

pub async fn worker_auth() {}

pub async fn worker_nofity() {}

pub async fn worker_subscrie() {}
