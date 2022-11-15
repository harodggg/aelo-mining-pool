use std::result;

use block::block_service::{Block, BlockService};
use block::{BlockRequest, BlockResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod block {
    tonic::include_proto!("block"); // The string specified here must match the proto package name
}
#[derive(Debug, Default)]
pub struct Block { }

#[tonic::async_trait]
impl Block { 
    async fn get_block(&self,request:BlockRequest) ->result<BlockResponse,Status>{
        Ok()
    }

}