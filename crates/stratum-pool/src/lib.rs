pub mod stratum_pool {
    tonic::include_proto!("stratum_pool");
}

use crate::stratum_pool::stratum_pool_server::{StratumPool, StratumPoolServer};
use crate::stratum_pool::{AuthorizeRequest, AuthorizeRespone};
use crate::stratum_pool::{ShareRequest, ShareRespone};
use crate::stratum_pool::{SubscribeRequest, SubscribeRespone};
use anyhow::Result;
use rand::{self, Rng};
use simple_log::info;
use std::net::SocketAddr;
use std::str::FromStr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct AleoStratumPool {}

#[tonic::async_trait]
impl StratumPool for AleoStratumPool {
    async fn mining_authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeRespone>, Status> {
        println!("{:?}", request);
        let respone = AuthorizeRespone {
            authorize_status: true,
        };
        Ok(Response::new(respone))
    }

    async fn mining_subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeRespone>, Status> {
        println!("{:?}", request);
        // parse socket addr
        let worker_rpc_server = request.get_ref();
        info!(
            "worker_rpc_server: {:?}",
            SocketAddr::from_str(&worker_rpc_server.worker_rpc_server)
        );

        //todo Geting respone date by trait，struct,or DB connect.
        //but now,we give simulated data，in the future,we will fix this.
        let mut rng = rand::thread_rng();
        let seq = rng.gen::<u128>();
        info!("Generate a serial number:{}", seq);

        let respone = SubscribeRespone {
            pool_name: "aleo_pool".to_string(),
            stratum_version: "AleoStratum/0.0.1".to_string(),
            subscription_number: seq.to_string(),
        };

        Ok(Response::new(respone))
    }

    async fn mining_share(
        &self,
        request: Request<ShareRequest>,
    ) -> Result<Response<ShareRespone>, Status> {
        println!("{:?}", request);
        let respone = ShareRespone { id: 123 };
        Ok(Response::new(respone))
    }
}

pub async fn run_stratum_service() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    println!("Starting Stratum Service");

    let stratum = AleoStratumPool::default();
    Server::builder()
        .add_service(StratumPoolServer::new(stratum))
        .serve(addr)
        .await?;

    Ok(())
}
