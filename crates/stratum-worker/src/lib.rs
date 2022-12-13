pub mod stratum_worker {
    tonic::include_proto!("stratum_worker");
}
use crate::stratum_worker::stratum_worker_server::{StratumWorker, StratumWorkerServer};
use crate::stratum_worker::{DifficultRequest, DifficultRespone};
use crate::stratum_worker::{NotifyRequest, NotifyRespone};
use anyhow::Result;
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
        //接受到挖矿通知，开始挖矿,建立任务队列。当接受新的通知时，终止旧的的任务。
        //有大量的逻辑处理逻辑。到此为止，所有的核心功能，基本开发完毕。
        //当挖出新的区块时，启动提交命令。

        let respone = NotifyRespone { status: 11 };

        Ok(Response::new(respone))
    }
}

pub async fn run_stratum_service() {
    let addr = "[::1]:50050".parse().unwrap();
    println!("Starting Stratum Service");

    let stratum = AleoStratumWorker::default();
    Server::builder()
        .add_service(StratumWorkerServer::new(stratum))
        .serve(addr)
        .await
        .unwrap();
}
