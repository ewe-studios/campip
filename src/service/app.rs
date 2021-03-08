use actix_web::{web, HttpResponse, Responder};

use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use log::{debug, info};
use std::net::IpAddr;
use std::str::FromStr;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::db::postgres;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{ip_address}").route(web::get().to(index)));
    return;
}

async fn index(ip_address: web::Path<String>, _pool: web::Data<Pool>) -> impl Responder {
    debug!("checking record for ip {}", ip_address.clone());

    let conn = &_pool.get().unwrap();
    postgres::get_location_by_ip_str(conn, ip_address.clone())
        .map(|location_data| {
            debug!("Found location: {:?}", location_data);
            HttpResponse::Ok().json(location_data)
        })
        .map_err(|_| {
            HttpResponse::NotFound().body(format!("target ip {} not found", ip_address.clone()))
        })
}
