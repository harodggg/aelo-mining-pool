pub mod stratum_worker {
    tonic::include_proto!("stratum_worker");
}
use anyhow::Result;
use stratum_worker::stratum_worker_server::{StratumWorker, StratumWorkerServer};
use stratum_worker::{DifficultRequest, DifficultRespone};
use stratum_worker::{NotifyRequest, NotifyRespone};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct AleoStratumWorker {}

#[tonic::async_trait]
impl StratumWorker for AleoStratumWorker {
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
}

pub async fn run_stratum_service() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    println!("Starting Stratum Service");

    let stratum = AleoStratumWorker::default();
    Server::builder()
        .add_service(StratumWorkerServer::new(stratum))
        .serve(addr)
        .await?;

    Ok(())
}
