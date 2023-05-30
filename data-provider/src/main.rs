mod datapoint;
mod meter;
mod reader;

use std::net::SocketAddr;

use meter::Meter;
use tonic::transport::Server;

use crate::meter::meter::meter_usage_service_server::MeterUsageServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let meter = Meter::default();

    Server::builder()
        .add_service(MeterUsageServiceServer::new(meter))
        .serve(addr)
        .await?;

    Ok(())
}
