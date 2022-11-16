use pool_service::block::block_server::{Block};
use pool_service::block::{BlockRequest, BlockRespone};
use tonic::{Request, Response, Status};

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
