mod meter;
mod reader;

use meter::Meter;
use tonic::transport::Server;

use crate::meter::meter::meter_usage_service_server::MeterUsageServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "0.0.0.0:3001".parse()?;
    let meter = Meter::default();

    Server::builder()
        .add_service(MeterUsageServiceServer::new(meter))
        .serve(addr)
        .await?;

    Ok(())
}
