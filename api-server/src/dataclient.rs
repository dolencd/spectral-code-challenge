use self::meter::{MeterUsage, MeterUsageDataPoint};
use crate::{
    dataclient::meter::meter_usage_service_client::MeterUsageServiceClient, SensorDataResponse,
};
use anyhow::Context;
use chrono::NaiveDateTime;

pub mod meter {
    tonic::include_proto!("meter_usage");
}

pub async fn fetch_sensor_data() -> anyhow::Result<MeterUsage> {
    let mut client = MeterUsageServiceClient::connect("http://localhost:3001").await?;

    let (_, response, _) = client.get_sensor_data(()).await?.into_parts();

    Ok(response)
}

impl TryFrom<MeterUsageDataPoint> for SensorDataResponse {
    type Error = anyhow::Error;
    fn try_from(value: MeterUsageDataPoint) -> Result<Self, Self::Error> {
        let input_timestamp = value.time.context("Missing time in data point")?;

        let timestamp = NaiveDateTime::from_timestamp_opt(
            input_timestamp.seconds,
            input_timestamp.nanos as u32,
        )
        .context("Failed to parse time in data point")?;

        Ok(SensorDataResponse {
            time: timestamp.to_string(),
            meterusage: value.value,
        })
    }
}
