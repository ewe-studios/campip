use chrono::Utc;
use diesel::pg::PgConnection;
use log::debug;

use csv::Reader;
use csv::StringRecord;

use crate::db::models::{Location, NewLocation};
use crate::db::postgres::{create_location, create_locations};

use crate::service::errors::AppError;
use std::error::Error;
use std::result::Result;

pub type CSVResult<T> = std::result::Result<T, AppError>;

fn transform_csv_string_record(record: &StringRecord) -> Result<NewLocation, Box<dyn Error>> {
    let from_ip_numeric = record.get(0).expect("expected from ip");
    let to_ip_numeric = record.get(1).expect("expected to ip");
    let country_code = record.get(2).expect("country_code expected");
    let country_name = record.get(3).expect("country_name expected");
    let region_name = record.get(4).expect("expected");
    let city = record.get(5).expect("expected");
    let zip_code = record.get(8).expect("expected");
    let time_zone = record.get(9).expect("expected");

    let latitude_f32: f32 = record.get(6).expect("expected").parse::<f32>()?;
    let longitude_f32 = record.get(7).expect("expected").parse::<f32>()?;

    let from_ip_num: i64 = from_ip_numeric.parse::<i64>()?;
    let to_ip_num: i64 = to_ip_numeric.parse::<i64>()?;

    Ok(NewLocation {
        type_: "".to_string(),
        street: "".to_string(),
        state: "".to_string(),
        postal: "".to_string(),
        zip: "".to_string(),
        city: city.to_string(),
        continent_code: "".to_string(),
        continent_name: "".to_string(),
        zip_code: zip_code.to_string(),
        country_code: country_code.to_string(),
        country_name: country_name.to_string(),
        region_code: "".to_string(),
        region_name: region_name.to_string(),
        time_zone: time_zone.to_string(),
        latitude: latitude_f32,
        longitude: longitude_f32,
        metro_code: "".to_string(),
        area_code: "".to_string(),
        from_ip: from_ip_numeric.to_string(),
        to_ip: to_ip_numeric.to_string(),
        from_ip_numeric: from_ip_num,
        to_ip_numeric: to_ip_num,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
    })
}

pub fn write_csv_to_db(conn: &PgConnection, source_file: &str, batch: usize) -> CSVResult<usize> {
    let mut reader = Reader::from_path(source_file)?;

    let mut created: usize = 0;
    let mut new_locations: Vec<NewLocation> = Vec::new();
    for record_result in reader.records() {
        let record = record_result.expect("should be a csv row");
        let new_loc = transform_csv_string_record(&record)?;
        debug!("Creating new location {:?}", &new_loc);

        new_locations.push(new_loc);
        if new_locations.len() > batch {
            let added = create_locations(conn, &new_locations)?;
            created += added;
            new_locations.clear()
        }
    }

    Ok(created)
}
