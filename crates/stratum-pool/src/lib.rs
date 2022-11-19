pub mod stratum_pool {
    tonic::include_proto!("stratum_pool");
}
use anyhow::Result;
use stratum_pool::stratum_pool_server::{StratumPool, StratumPoolServer};
use stratum_pool::{AuthorizeRequest, AuthorizeRespone};
use stratum_pool::{DifficultRequest, DifficultRespone};
use stratum_pool::{NotifyRequest, NotifyRespone};
use stratum_pool::{ShareRequest, ShareRespone};
use stratum_pool::{SubscribeRequest, SubscribeRespone};
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
        let respone = AuthorizeRespone { id: 1 };
        Ok(Response::new(respone))
    }

    async fn mining_subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeRespone>, Status> {
        println!("{:?}", request);
        let respone = SubscribeRespone { id: 1 };
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
