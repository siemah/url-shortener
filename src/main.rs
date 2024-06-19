mod config;
mod router;
mod routes;
mod utils;

use libsql::{Builder, Database};
use std::error::Error;
use actix_web::{App, HttpServer, Responder, web};
use log::info;
use serde::Deserialize;
use crate::config::Config;
use crate::router::init_router;
use crate::routes::{create_shorten_url, redirect};


#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let Config { port, .. } = Config::new_connection().await?;
    let addr = format!("localhost:{port}");
    info!("Server running on port http://{addr}");
    let server = init_router(addr).await;
    Ok(server)
}

