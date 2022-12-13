use std::cell::RefCell;

use super::block::block_server::{Block, BlockServer};
use super::block::{BlockRequest, BlockRespone};
use crate::block_observer::BlockObserver;
use anyhow::Result;
use simple_log::info;
use snarkvm::prelude::{EpochChallenge, FromBytes, Network, Testnet3};
use stratum_pool::{stratum_pool::stratum_pool_server::StratumPoolServer, AleoStratumPool};
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tonic::codegen::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// Use a separate green thread.Operate independently
#[allow(dead_code)]
#[derive(Debug)]
pub struct AleoBlock {
    epoch_challenge: Arc<RwLock<EpochChallenge<Testnet3>>>,
    observers: Arc<RwLock<BlockObserver>>,
    sender: Arc<RwLock<Sender<EpochChallenge<Testnet3>>>>,
}

impl AleoBlock {
    pub fn default(tx: Sender<EpochChallenge<Testnet3>>) -> Self {
        Self {
            epoch_challenge: Arc::new(RwLock::new(
                EpochChallenge::<Testnet3>::new(0, <Testnet3 as Network>::BlockHash::default(), 3)
                    .unwrap(),
            )),
            observers: Arc::new(RwLock::new(BlockObserver::default())),
            sender: Arc::new(RwLock::new(tx)),
        }
    }
}

#[tonic::async_trait]
impl Block for AleoBlock {
    async fn submit_latest_block(
        &self,
        request: Request<BlockRequest>,
    ) -> Result<Response<BlockRespone>, Status> {
        info!("Get BlockRequest from Aleo Client :{:?}", request.get_ref());
        self.observers.read().await;

        // let _ = &self.block;
        // self.hello();
        let epoch_challenge =
            EpochChallenge::<Testnet3>::from_bytes_le(&request.get_ref().epoch_challenge);
        // EpochChallenge::<Testnet3>::from_bytes_le(&request.get_ref().epoch_challenge);
        // info!("epoch_challenge raw result: {:?}", epoch_challenge;
        let mut epoch_challenge_lock = self.epoch_challenge.write().await;
        let sender_lock = self.sender.write().await;
        if let Ok(epoch_challenge) = epoch_challenge {
            // info!("epoch_challenge raw: {:?}", epoch_challenge);
            *epoch_challenge_lock = epoch_challenge.clone();

            // write epoch_challenge to reciver by sender
            sender_lock.send(epoch_challenge.clone()).await.unwrap();
        }

        // write epoch_challenge to reciver by sender
        // let sender_lock = self.sender.write().await;
        // if let Ok(epoch_challenge) = epoch_challenge {
        //     sender_lock.send(epoch_challenge).await;
        // }

        // info!(
        //     "self epoch_challenge {:?}",
        //     self.epoch_challenge.read().await
        // );
        // todo 通知所有的观察者。观察者，通知所有的worker。worker 从数据库获取数据。
        // todo 建立新的平行线程，通过aleo worker 的rpc 通知。
        {}
        let response = BlockRespone { status: 1 };

        Ok(Response::new(response))
    }
}

#[allow(dead_code)]
impl AleoBlock {
    fn get_block() -> Result<String> {
        Ok(String::from("hellO"))
    }
    fn update_block(&self) -> Result<bool> {
        Ok(true)
    }
    fn hello(&self) {}
}

pub async fn run_rpc(tx: Sender<EpochChallenge<Testnet3>>) {
    let block = AleoBlock::default(tx);
    let stratum_pool = AleoStratumPool::default();

    let addr = "[::1]:50051".parse().unwrap();
    info!("Starting Stratum Service");
    Server::builder()
        .add_service(BlockServer::new(block))
        .add_service(StratumPoolServer::new(stratum_pool))
        .serve(addr)
        .await
        .unwrap();
}
