use crate::client::block::{block_client::BlockClient, BlockRequest};
use crate::client::rpc;
use anyhow::Result;
use std::time::Duration;
use tokio;
use tonic::{transport::Channel, Request};

pub struct ClientRpc(BlockClient<Channel>);
impl ClientRpc {
    pub fn new(block_client: BlockClient<Channel>) -> Self {
        rpc::ClientRpc(block_client)
    }

    pub async fn request_block(&mut self) {
        let request = tonic::Request::new(BlockRequest {
            timestramp: 1,
            coinbase_target: 2,
            proof_target: 3,
        });
        let response = self
            .0
            .submit_latest_block(request)
            .await
            .expect("sumbit error block");
        println!("RESPONSE={:#?}", response.get_ref());
        tokio::time::sleep(Duration::from_secs(30)).await;
    }

    pub async fn run() -> Result<()> {
        loop {
            let mut client = BlockClient::connect("http://[::1]:50051").await?;
            let request = tonic::Request::new(BlockRequest {
                timestramp: 1,
                coinbase_target: 2,
                proof_target: 3,
            });
            let response = client.submit_latest_block(request).await?;
            println!("RESPONSE={:#?}", response.get_ref());
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }
}
