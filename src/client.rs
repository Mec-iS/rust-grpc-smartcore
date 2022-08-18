pub mod servicebase {
    tonic::include_proto!("servicebase");
}

use std::str::FromStr;
use servicebase::service_base_client::ServiceBaseClient;
use servicebase::{ComputeItem, DMatrix};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServiceBaseClient::connect("http://0.0.0.0:5005").await?;

    let request = tonic::Request::new(());

    let response = client.get_available(request).await?;

    println!("{:?}", response.into_inner().available);

    let request = tonic::Request::new(
        ComputeItem {
            x : Some(DMatrix {
                rows: 6,
                columns: 3,
                array: vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                            1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                            1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            }),
            y : Some(DMatrix {
                rows: 1,
                columns: 3,
                array: vec![0.313324, 0.23423, 0.51234, 0.42345, 0.3234, 0.2342],
            }),
            module: String::from_str("linear::linear_regression::LinearRegression").unwrap(),
            operation: String::from_str("fit_predict").unwrap(),
            uid: 1,        
        }
    );

    let response = client.submit_compute(request).await?;

    println!("{:?}", response.into_inner().result);

    Ok(())
}