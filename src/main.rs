//! src/main.rs
// use log::{debug, error, info, warn};
use log::info;
use simple_logger::SimpleLogger;
use std::net::TcpListener;
use sqlx::{Connection, PgConnection, ConnectOptions};

use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = connection_string
    .connect()
    .await
    .expect("Failed to connect to Postgres.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    info!("Starting service");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}