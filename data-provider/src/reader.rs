use anyhow::Result;
use std::io::Cursor;

use crate::datapoint::DataPoint;


pub fn read_csv_sensor_data() -> Result<Vec<DataPoint>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(include_bytes!("./meterusage.csv")));

    let result: Result<Vec<DataPoint>> = reader
        .records()
        .map(|record| DataPoint::try_from(record?))
        .collect();
    result
}
