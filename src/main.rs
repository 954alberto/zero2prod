//! src/main.rs
// use log::{debug, error, info, warn};
use log::info;
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPoolOptions;
use sqlx::{ConnectOptions, PgPool};
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();

    let configuration = get_configuration().expect("Failed to read configuration");

    let pool = PgPoolOptions::new()
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres using connection pool.");

    info!("Starting service");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await
}
