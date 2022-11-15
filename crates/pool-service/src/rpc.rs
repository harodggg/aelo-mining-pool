pub mod block {
    tonic::include_proto!("block"); // The string specified here must match the proto package name
}

use block::block_server::{Block, BlockServer};
use block::{BlockRequest, BlockRespone};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct AleoBlock {}

#[tonic::async_trait]
impl Block for AleoBlock {
    async fn get_block(
        &self,
        request: Request<BlockRequest>,
    ) -> Result<Response<BlockRespone>, Status> {
        println!("{:?}", request);
        let response = BlockRespone { difficult: 11 };
        Ok(Response::new(response))
    }
}