use super::block::block_server::{Block, BlockServer};
use super::block::{BlockRequest, BlockRespone};
use anyhow::Result;
use simple_log::info;
use snarkvm::prelude::{EpochChallenge, FromBytes, Network, Testnet3};
use tokio::sync::RwLock;
use tonic::codegen::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct AleoBlock {
    epoch_challenge: Arc<RwLock<EpochChallenge<Testnet3>>>,
}

impl AleoBlock {
    pub fn default() -> Self {
        Self {
            epoch_challenge: Arc::new(RwLock::new(
                EpochChallenge::<Testnet3>::new(0, <Testnet3 as Network>::BlockHash::default(), 3)
                    .unwrap(),
            )),
        }
    }
}

#[tonic::async_trait]
impl Block for AleoBlock {
    async fn submit_latest_block(
        &self,
        request: Request<BlockRequest>,
    ) -> Result<Response<BlockRespone>, Status> {
        info!("{:#?}", request.get_ref());
        // let _ = &self.block;
        // self.hello();
        let epoch_challenge =
            EpochChallenge::<Testnet3>::from_bytes_le(&request.get_ref().epoch_challenge);
        let mut epoch_challenge_lock = self.epoch_challenge.write().await;
        if let Ok(epoch_challenge) = epoch_challenge {
            *epoch_challenge_lock = epoch_challenge;
        }
        info!("{:?}", self.epoch_challenge);
        let response = BlockRespone { status: 1 };
        Ok(Response::new(response))
    }
}

impl AleoBlock {
    fn get_block() -> Result<String> {
        Ok(String::from("hellO"))
    }
    fn update_block(&self) -> Result<bool> {
        Ok(true)
    }
    fn hello(&self) {}
}

pub async fn run_aleo_block() {
    let block = AleoBlock::default();
    let addr = "[::1]:50051".parse().unwrap();
    info!("Starting Stratum Service");
    Server::builder()
        .add_service(BlockServer::new(block))
        .serve(addr)
        .await
        .unwrap();
}
