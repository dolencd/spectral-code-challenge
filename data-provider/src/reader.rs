use anyhow::Result;
use chrono::NaiveDateTime;

use crate::datapoint::DataPoint;

pub fn read_csv_sensor_data() -> impl Iterator<Item = DataPoint> {
    include_str!("./meterusage.csv")
        .lines()
        .skip(1)
        .map_while(|row| DataPoint::try_from_csv_row(row).ok())
}

impl DataPoint {
    fn try_from_csv_row(row: &str) -> Result<DataPoint> {
        {
            let rows = row.split(',').collect::<Vec<&str>>();
            Ok(match rows[..] {
                [timestamp, value] => DataPoint {
                    meter_usage: value.parse()?,
                    time: NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")?,
                },
                _ => todo!(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::datapoint::DataPoint;

    use super::read_csv_sensor_data;

    #[test]
    fn can_read_lines() {
        let mut iterator = read_csv_sensor_data();
        assert_eq!(
            iterator.next().unwrap(),
            DataPoint {
                meter_usage: 55.09,
                time: NaiveDateTime::parse_from_str("2019-01-01 00:15:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
            }
        )
    }

    #[test]
    fn parses_good_values() {
        assert_eq!(
            DataPoint::try_from_csv_row("2019-01-01 00:15:00,55.09").unwrap(),
            DataPoint {
                meter_usage: 55.09,
                time: NaiveDateTime::parse_from_str("2019-01-01 00:15:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
            }
        )
    }

    #[test]
    fn fails_on_invalid_date() {
        assert!(DataPoint::try_from_csv_row("2019-13-13 00:15:00,55.09").is_err())
    }

    #[test]
    fn fails_on_invalid_value() {
        assert!(DataPoint::try_from_csv_row("2019-01-01 00:15:00,5a.09").is_err())
    }
}
