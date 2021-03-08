use crate::service::errors::AppError;
use diesel::debug_query;
use diesel::pg::upsert::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use log::debug;

use crate::db::models::{Location, NewLocation};
use crate::schema::locations;
use crate::utils::net;
use diesel::expression::ops::Add;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type AppResult<T> = std::result::Result<T, AppError>;

pub fn create_locations(conn: &PgConnection, loc: &Vec<NewLocation>) -> AppResult<usize> {
    conn.transaction(move || {
        diesel::insert_into(locations::table)
            .values(loc)
            .on_conflict((locations::from_ip_numeric, locations::to_ip_numeric))
            .do_nothing()
            .execute(conn)
            .map_err(AppError::from)
    })
}

pub fn create_location(conn: &PgConnection, loc: NewLocation) -> AppResult<usize> {
    conn.transaction(move || {
        diesel::insert_into(locations::table)
            .values(&loc)
            .on_conflict((locations::from_ip_numeric, locations::to_ip_numeric))
            .do_nothing()
            .execute(conn)
            .map_err(AppError::from)
    })
}

pub fn get_location_for_ip_in_decimal(
    conn: &PgConnection,
    ip_in_decimal: i64,
) -> AppResult<Location> {
    debug!(
        "Checking for ip in decimal notation {} from datastore",
        ip_in_decimal
    );
    conn.transaction(move || {
        locations::table
            .filter(locations::to_ip_numeric.ge(ip_in_decimal))
            .select(locations::all_columns)
            .first(conn)
            .map_err(AppError::from)
    })
}

pub fn get_location_by_ip(conn: &PgConnection, ip: IpAddr) -> AppResult<Location> {
    debug!("Checking for ip addr {} from datastore", ip);
    match ip {
        IpAddr::V4(ip) => {
            let ip_dec = net::from_ipv4(ip)?;
            debug!("Checking for ip4 addr {} from datastore", ip_dec);
            get_location_for_ip_in_decimal(conn, ip_dec)
        }
        IpAddr::V6(ip) => {
            let ip_dec = net::from_ipv6(ip)?;
            debug!("Checking for ip6 addr {} from datastore", ip_dec);
            get_location_for_ip_in_decimal(conn, ip_dec)
        }
    }
}

pub fn get_location_by_ip_str(conn: &PgConnection, ip: String) -> AppResult<Location> {
    debug!("Checking for ip {} from datastore", ip);
    let target_ip = IpAddr::from_str(ip.as_str())?;
    get_location_by_ip(conn, target_ip)
}

pub fn get_location_by_id(conn: &PgConnection, location_id: i32) -> AppResult<Location> {
    conn.transaction(move || {
        locations::table
            .find(location_id)
            .select(locations::all_columns)
            .first(conn)
            .map_err(AppError::from)
    })
}

pub async fn create_database_pool(database_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("should have  create pg connection manager")
}

pub async fn create_pg_connection(database_url: String) -> PgConnection {
    PgConnection::establish(database_url.as_str()).expect("error creating connection")
}
