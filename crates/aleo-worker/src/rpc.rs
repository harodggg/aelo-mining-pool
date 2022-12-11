use anyhow::Result;
use stratum_pool::stratum_pool::{stratum_pool_client::StratumPoolClient, SubscribeRequest};
use tokio::time::Duration;
use tonic;

// auth login
// worker -> rpc service -> prover -> worker -> status
pub async fn rpc_client_run() -> Result<()> {
    loop {
        let mut client = StratumPoolClient::connect("http://[::1]:50050").await?;
        let request = tonic::Request::new(SubscribeRequest { id: 1 });
        let response = client.mining_subscribe(request).await?;
        println!("RESPONSE={:#?}", response.get_ref());
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

// submit block
pub async fn submit_block() -> Result<bool> {
    Ok(true)
}
