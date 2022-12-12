pub mod stratum_pool {
    tonic::include_proto!("stratum_pool");
}
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::stratum_pool::stratum_pool_server::{StratumPool, StratumPoolServer};
use crate::stratum_pool::{AuthorizeRequest, AuthorizeRespone};
use crate::stratum_pool::{DifficultRequest, DifficultRespone};
use crate::stratum_pool::{NotifyRequest, NotifyRespone};
use crate::stratum_pool::{ShareRequest, ShareRespone};
use crate::stratum_pool::{SubscribeRequest, SubscribeRespone};
use anyhow::Result;
use async_trait::async_trait;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
#[derive(Debug, Default)]
pub struct AleoStratumPool {}

impl AleoStratumPool {}

#[async_trait]
trait WorkerAble {}

#[async_trait]
impl WorkerAble for AleoStratumPool {}

#[tonic::async_trait]
impl StratumPool for AleoStratumPool {
    async fn mining_authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeRespone>, Status> {
        println!("{:?}", request);
        let respone = AuthorizeRespone { id: 1 };
        Ok(Response::new(respone))
    }

    async fn mining_subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeRespone>, Status> {
        println!("{:?}", request);
        //todo Geting respone date by trait，struct,or DB connect.
        //but now,we give simulated data，in the future,we will fix this.

        let mut hasher = DefaultHasher::new();
        hasher.write(b"sub success");

        let respone = SubscribeRespone {
            pool_name: "aleo_pool".to_string(),
            stratum_version: "0.0.1".to_string(),
            subscription_number: hasher.finish().to_string(),
        };

        Ok(Response::new(respone))
    }

    async fn mining_set_difficult(
        &self,
        request: Request<DifficultRequest>,
    ) -> Result<Response<DifficultRespone>, Status> {
        println!("{:?}", request);
        let respone = DifficultRespone { id: 1 };
        Ok(Response::new(respone))
    }

    async fn mining_notify(
        &self,
        request: Request<NotifyRequest>,
    ) -> Result<Response<NotifyRespone>, Status> {
        println!("{:?}", request);
        let respone = NotifyRespone { id: 1 };
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
