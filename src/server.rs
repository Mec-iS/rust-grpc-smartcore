pub mod servicebase {
    tonic::include_proto!("servicebase");
}

use std::str::FromStr;
use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use servicebase::service_base_server::{ServiceBaseServer, ServiceBase};
use servicebase::{ComputeItem, DMatrix, AvailableCompute, GetAvailableResponse, Results};

use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linear::linear_regression::{LinearRegression, LinearRegressionParameters, LinearRegressionSolverName};

#[derive(Debug, Default)]
pub struct SmartcoreService {

    /// store queue of ids in processing
    processing: Mutex<Vec<i32>>,
    /// store a constant with the list of available modules and operations
    available: Vec<(String, String)>,
}

#[tonic::async_trait]
impl ServiceBase for SmartcoreService {
    async fn get_available(&self, _: Request<()>) -> Result<Response<GetAvailableResponse>, Status> {
        let message = GetAvailableResponse {
            available: vec![AvailableCompute {
                module: String::from_str("linear::linear_regression::LinearRegression").unwrap(),
                operation: String::from_str("fit_predict").unwrap(),
            },]
        };

        Ok(Response::new(message))
    }

    async fn submit_compute(&self, request: Request<ComputeItem>) -> Result<Response<Results>, Status> {
        let payload = request.into_inner();

        let module: String = payload.module.into();
        let dmatrix_x = payload.x.unwrap();
        let x_rows = dmatrix_x.rows.clone();
        let x_cols = dmatrix_x.columns.clone();

        let x: Vec<f64> = dmatrix_x.array.into();
        let y: Vec<f64> = payload.y.unwrap().array.into();
        
        let _X = DenseMatrix::from_vec(
            x_rows as usize,
            x_cols as usize,
            &x
        );
    
        let results;
        match &module[..] {
            "linear::linear_regression::LinearRegression" => 
                 results = Some(
                    LinearRegression::fit(
                        &_X,
                        &y,
                        LinearRegressionParameters {
                            solver: LinearRegressionSolverName::QR,
                        },
                    ).and_then(|lr| lr.predict(&_X))
                    .unwrap()),
            _ => results = None,
        };

        let results = results.unwrap();
        
        let message = Results {
            module: module,
            operation: payload.operation.into(),
            result: Some(DMatrix {
                rows: 1,
                columns: 5,
                array: results
            }),
        };

        Ok(Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5005".parse().unwrap();
    let service = SmartcoreService::default();

    let (shutdown_trigger, shutdown_signal) = triggered::trigger();

    // A sync `Fn` closure will trigger the trigger when the user hits Ctrl-C
    ctrlc::set_handler(move || {
        shutdown_trigger.trigger();
    }).expect("Error setting Ctrl-C handler");

    println!("Running at {:?}", addr);

    Server::builder()
        .add_service(ServiceBaseServer::new(service))
        .serve_with_shutdown(addr, shutdown_signal)
        .await?;

    Ok(())
}
