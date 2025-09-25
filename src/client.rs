use tonic::Request;

pub mod bitcoin {
    tonic::include_proto!("bitcoin");
}

use bitcoin::bitcoin_client::BitcoinClient;
use bitcoin::BitcoinRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = Request::new(BitcoinRequest { a: 56 });

    let response = client.update_price(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
