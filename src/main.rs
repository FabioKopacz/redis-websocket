use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod bitcoin {
    tonic::include_proto!("bitcoin");
}

use bitcoin::bitcoin_server::{Bitcoin, BitcoinServer};
use bitcoin::{BitcoinReply, BitcoinRequest};

#[derive(Debug, Default)]
pub struct MyBitcoin {}

#[tonic::async_trait]
impl Bitcoin for MyBitcoin {
    async fn update_price(
        &self,
        request: Request<BitcoinRequest>,
    ) -> Result<Response<BitcoinReply>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        println!("Got a request: {:?}", req.a);

        let reply = BitcoinReply {
            resultado: req.a * 2,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyBitcoin::default();

    Server::builder()
        .add_service(BitcoinServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
