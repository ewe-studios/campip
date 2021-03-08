#[macro_use]
extern crate log;

use std::env;
use std::io;

use dotenv::dotenv;
use env_logger::{Builder, Target};
use std::env::args;

type Result = io::Result<()>;

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

#[actix_web::main]
async fn main() -> Result {
    dotenv().ok();

    env_logger::init();

    let cmd = args()
        .nth(1)
        .expect("expected command to be provided")
        .parse::<String>()
        .expect("invalid value type");

    if cmd == "serve" {
        let addr = env::var("ADDR").expect("ADDR to be set");
        let database_url = env::var("DATABASE_URL").expect("DB_URL to be set");
        let port_str: String = env::var("PORT").expect("PORT to be set");
        let port: u16 = port_str.parse::<u16>().unwrap();

        println!("Starting webserver on {}:{:?}", addr, port);
        match campip::run(port, addr, database_url, LOG_FORMAT.parse().unwrap()).await {
            Ok(()) => {
                println!("bye!")
            }
            Err(error) => {
                panic!("failed to start server: {}", error)
            }
        }
    }

    if cmd == "from_csv" {
        let csv_file = args()
            .nth(2)
            .expect("expected file path to be provided")
            .parse::<String>()
            .expect("invalid value type");

        let database_url = env::var("DATABASE_URL").expect("DB_URL to be set");
        match campip::copy_from_csv(database_url, csv_file, 1000).await {
            Ok(count) => {
                println!("Copied {} into database", count)
            }
            Err(error) => {
                panic!("failed to move csv files over: {}", error)
            }
        }
    }

    println!("Not yet implemented");
    Ok(())
}
