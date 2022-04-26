use crate::schema::locations;
use chrono::prelude::*;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "locations"]
pub struct NewLocation {
    pub type_: String,
    pub continent_code: String,
    pub continent_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal: String,
    pub zip: String,
    pub zip_code: String,
    pub country_code: String,
    pub country_name: String,
    pub region_code: String,
    pub region_name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub metro_code: String,
    pub time_zone: String,
    pub area_code: String,
    pub from_ip: String,
    pub to_ip: String,
    pub from_ip_numeric: i64,
    pub to_ip_numeric: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Location {
    pub id: i32,
    pub type_: String,
    pub continent_code: String,
    pub continent_name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal: String,
    pub zip: String,
    pub zip_code: String,
    pub country_code: String,
    pub country_name: String,
    pub region_code: String,
    pub region_name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub metro_code: String,
    pub time_zone: String,
    pub area_code: String,
    pub from_ip: String,
    pub to_ip: String,
    pub from_ip_numeric: i64,
    pub to_ip_numeric: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct LocationResponse {
    pub ip: String,
    pub location: Location,
}
