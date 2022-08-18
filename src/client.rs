pub mod servicebase {
    tonic::include_proto!("servicebase");
}

use servicebase::service_base_client::ServiceBaseClient;
use servicebase::{GetAvailableResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServiceBaseClient::connect("http://0.0.0.0:5005").await?;

    let request = tonic::Request::new(());

    let response = client.get_available(request).await?;

    println!("{:?}", response.into_inner().available);

    Ok(())
}