use super::block::block_server::{Block, BlockServer};
use super::block::{BlockRequest, BlockRespone};
use anyhow::Result;
use tonic::codegen::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct AleoBlock {
    block: Arc<String>,
}

#[tonic::async_trait]
impl Block for AleoBlock {
    async fn submit_latest_block(
        &self,
        request: Request<BlockRequest>,
    ) -> Result<Response<BlockRespone>, Status> {
        println!("{:#?}", request.get_ref());
        // let _ = &self.block;
        // self.hello();
<<<<<<< HEAD
        let response = BlockRespone { status: 1 };
=======
        let response = BlockRespone { height: 1 };
>>>>>>> main
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
    println!("Starting Stratum Service");
    Server::builder()
        .add_service(BlockServer::new(block))
        .serve(addr)
        .await
        .unwrap();
}
