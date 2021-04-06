#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use std::io;

mod data;
mod db;
mod schema;
mod service;
mod utils;

use crate::data::csv::{write_csv_to_db, CSVResult};
use crate::db::postgres::{create_database_pool, create_pg_connection};
use diesel::PgConnection;


pub async fn run(
    port: u16,
    addr: String,
    database_url: String,
    log_format: String,
) -> io::Result<()> {
    let pool = create_database_pool(database_url).await;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::new(log_format.as_str()))
            .configure(service::app::configure)
    })
    .bind((addr.as_str(), port))?
    .run()
    .await
}

pub async fn copy_from_csv(
    database_url: String,
    csv_file: String,
    batch: usize,
) -> CSVResult<usize> {
    let database_conn = create_pg_connection(database_url).await;
    let records = write_csv_to_db(&database_conn, csv_file.as_str(), batch)?;
    Ok(records)
}

pub async fn get_db(database_url: String) -> PgConnection {
    create_pg_connection(database_url).await
}
