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
                rows: 12,
                columns: 6,
                array: vec![
                    234.289, 235.6, 159.0, 107.608, 1947., 60.323,
                    258.054, 368.2, 161.6, 109.773, 1949., 60.171,
                    284.599, 335.1, 165.0, 110.929, 1950., 61.187,
                    328.975, 209.9, 309.9, 112.075, 1951., 63.221,
                    397.469, 290.4, 304.8, 117.388, 1955., 66.019,
                    419.180, 282.2, 285.7, 118.734, 1956., 67.857,
                    442.769, 293.6, 279.8, 120.445, 1957., 68.169,
                    444.546, 468.1, 263.7, 121.950, 1958., 66.513,
                    482.704, 381.3, 255.2, 123.366, 1959., 68.655,
                    502.601, 393.1, 251.4, 125.368, 1960., 69.564,
                    518.173, 480.6, 257.2, 127.852, 1961., 69.331,
                    554.894, 400.7, 282.7, 130.081, 1962., 70.551,
                ],
            }),
            y : Some(DMatrix {
                rows: 1,
                columns: 12,
                array: vec![
                    83.0, 88.5, 88.2, 89.5, 96.2, 98.1, 99.0, 100.0, 101.2, 104.6, 108.4, 110.8,
                ],
            }),
            module: String::from_str("linear::linear_regression::LinearRegression").unwrap(),
            operation: String::from_str("fit_predict").unwrap(),
            uid: 1,        
        }
    );

    let response = client.submit_compute(request).await?;

    println!("{:?}", response.into_inner().result);

    let request = tonic::Request::new(
        ComputeItem {
            x : Some(DMatrix {
                rows: 3,
                columns: 3,
                array: vec![1.0, 1.0, 1.0,
                            1.0, 1.0, 1.0,
                            1.0, 1.0, 1.0],
            }),
            y : Some(DMatrix {
                rows: 1,
                columns: 3,
                array: vec![0.313324, 0.23423, 0.51234],
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