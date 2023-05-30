use chrono::NaiveDateTime;
use csv::StringRecord;
use prost_types::Timestamp;

use crate::meter::meter::MeterUsageDataPoint;

#[derive(Debug)]
pub struct DataPoint {
    pub time: NaiveDateTime,
    pub meter_usage: f32,
}

impl TryFrom<StringRecord> for DataPoint {
    type Error = anyhow::Error;
    fn try_from(value: StringRecord) -> std::result::Result<Self, Self::Error> {
        Ok(DataPoint {
            // NaiveDateTime just means that it doesn't have a timezone
            time: NaiveDateTime::parse_from_str(&value[0], "%Y-%m-%d %H:%M:%S")?,
            meter_usage: value[1].parse()?,
        })
    }
}

impl Into<MeterUsageDataPoint> for DataPoint {
    fn into(self) -> MeterUsageDataPoint {
        MeterUsageDataPoint {
            time: Some(Timestamp {
                seconds: self.time.timestamp(),
                nanos: self.time.timestamp_subsec_nanos() as i32,
            }),
            value: self.meter_usage,
        }
    }
}
