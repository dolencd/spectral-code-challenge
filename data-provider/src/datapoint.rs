use chrono::NaiveDateTime;
use prost_types::Timestamp;

use crate::meter::meter::MeterUsageDataPoint;

#[derive(Debug)]
pub struct DataPoint {
    pub time: NaiveDateTime,
    pub meter_usage: f32,
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

impl PartialEq for DataPoint {
    fn eq(&self, other: &Self) -> bool {
        self.meter_usage == other.meter_usage && self.time == other.time
    }
}
