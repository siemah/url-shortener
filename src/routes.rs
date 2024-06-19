use actix_web::{Responder, web};
use log::info;
use serde::Deserialize;
use crate::config::Config;
use crate::utils;

#[derive(Deserialize)]
pub struct ShortenUrlBody {
    pub url: String,
}

pub async fn index() -> String {
    "Azul felawen!".to_string()
}

pub async fn create_shorten_url(body: web::Json<ShortenUrlBody>) -> impl Responder {
    info!("Creating new short url: {:#?}", body.url);
    let Config { db, ..} = Config::new_connection().await.expect("Failed to connect to database");
    let db_connection = db.connect().expect("Failed to connect to database");
    db_connection
        .query(
            "INSERT INTO urls (url) VALUES (?1) RETURNING id",
            vec![body.url.clone()]
        )
        .await
        .expect("Failed to insert new url");
    let row_id = db_connection.last_insert_rowid();
    let short_url_path = utils::convert_to_id(row_id);
    println!("new row id is: {row_id} path {}", short_url_path);
    // todo: generate a new path for the short url using its id
    db_connection.query(
        "update urls set path=(?1) where id = (?2)",
        vec![short_url_path, row_id.to_string()]
    )
        .await
        .expect("Failed to insert new url");

    "Hello, world!"
}
