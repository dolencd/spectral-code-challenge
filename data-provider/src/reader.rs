use anyhow::Result;
use chrono::NaiveDateTime;
use csv::StringRecord;
use std::io::Cursor;

#[derive(Debug)]
pub struct DataPoint {
    pub time: NaiveDateTime,
    pub meter_usage: f32,
}

impl TryFrom<StringRecord> for DataPoint {
    type Error = anyhow::Error;
    fn try_from(value: StringRecord) -> std::result::Result<Self, Self::Error> {
        Ok(DataPoint {
            time: NaiveDateTime::parse_from_str(&value[0], "%Y-%m-%d %H:%M:%S")?,
            meter_usage: value[1].parse().expect("Failed to parse meter_usage f32"),
        })
    }
}

pub fn read_csv_sensor_data() -> Result<Vec<DataPoint>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(include_bytes!("./meterusage.csv")));

    let result: Result<Vec<DataPoint>> = reader
        .records()
        .map(|record| DataPoint::try_from(record?))
        .collect();
    result
}
