//! src/main.rs
use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;
use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    info!("Starting service");
    let address = format!("127.0.0.1:0");
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}