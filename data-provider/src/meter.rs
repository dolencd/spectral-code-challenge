use meter::meter_usage_service_server::MeterUsageService;
use meter::MeterUsage;
use tonic::{Request, Response, Status};

use crate::reader::{read_csv_sensor_data};

pub mod meter {
    tonic::include_proto!("meter_usage");
}

#[derive(Debug, Default)]
pub struct Meter {}

#[tonic::async_trait]
impl MeterUsageService for Meter {
    async fn get_sensor_data(&self, request: Request<()>) -> Result<Response<MeterUsage>, Status> {
        println!("Got a request: {:?}", request);

        let reply = MeterUsage {
            meter_usage: read_csv_sensor_data()
                .map_err(|err| {
                    eprintln!("Failed to read sensor data {:?}", err);
                    Status::internal("Failed to read sensor data")
                })?
                .into_iter()
                .map(|data_point| data_point.into())
                .collect(),
        };

        Ok(Response::new(reply))
    }
}
