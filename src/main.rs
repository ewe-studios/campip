#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;
use std::io;

use dotenv::dotenv;
use diesel_migrations::embed_migrations;
use env_logger::{Builder, Target};
use std::env::args;

type Result = io::Result<()>;
type MigrationResult = std::result::Result<(), diesel_migrations::RunMigrationsError>;

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

embed_migrations!();

pub async fn setup_db(database_url: String) -> MigrationResult {
    let database_conn = campip::get_db(database_url).await;
    embedded_migrations::run_with_output(&database_conn, &mut std::io::stdout())
}

#[actix_web::main]
async fn main() -> Result {
    dotenv().ok();

    env_logger::init();

	let postgres_username = env::var("POSTGRES_USER").expect("must be provided");
	let postgres_password = env::var("POSTGRES_PASSWORD").expect("must be provided");
	let postgres_addr = env::var("POSTGRES_ADDR").expect("must be provided");
	let postgres_db = env::var("POSTGRES_DB").expect("must be provided");

	let database_url = format!("postgres://{}:{}@{}/{}", postgres_username, postgres_password, postgres_addr, postgres_db);


    let cmd = args()
        .nth(1)
        .expect("expected command to be provided")
        .parse::<String>()
        .expect("invalid value type");

	if cmd == "setup" {
        match setup_db(database_url.clone()).await {
            Ok(()) => {
                println!("Finished!")
            }
            Err(error) => {
                panic!("failed to run migrations: {}", error);
            }
        }
	}

    if cmd == "serve" {
        let addr = env::var("ADDR").expect("ADDR to be set");

        let port_str: String = env::var("PORT").expect("PORT to be set");
        let port: u16 = port_str.parse::<u16>().unwrap();

        println!("Starting webserver on {}:{:?} and connecting to db on {}", addr, port, database_url);
        match campip::run(port, addr, database_url.clone(), LOG_FORMAT.parse().unwrap()).await {
            Ok(()) => {
                println!("bye!")
            }
            Err(error) => {
                panic!("failed to start server: {}", error)
            }
        };
    }

    if cmd == "from_csv" {
        let csv_file = args()
            .nth(2)
            .expect("expected file path to be provided")
            .parse::<String>()
            .expect("invalid value type");

        match campip::copy_from_csv(database_url.clone(), csv_file, 1000).await {
            Ok(count) => {
                println!("Copied {} into database", count)
            }
            Err(error) => {
                panic!("failed to move csv files over: {}", error)
            }
        };
    }

    println!("unknown command");
    Ok(())
}
