pub mod stratum {
    tonic::include_proto!("stratum");
}
use stratum::stratum_server::{Stratum, StratumServer};
use stratum::{AuthorizeRequest, AuthorizeRespone};
use stratum::{DifficultRequest, DifficultRespone};
use stratum::{NotifyRequest, NotifyRespone};
use stratum::{ShareRequest, ShareRespone};
use stratum::{SubscribeRequest, SubscribeRespone};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct AleoStratum {}

#[tonic::async_trait]
impl Stratum for AleoStratum {
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

pub async fn run_stratum_service() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Starting Stratum Service");

    let stratum = AleoStratum::default();
    Server::builder()
        .add_service(StratumServer::new(stratum))
        .serve(addr)
        .await?;

    Ok(())
}
