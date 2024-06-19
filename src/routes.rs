use actix_web::{HttpResponse, Responder, web};
use libsql::Row;
use log::info;
use serde::Deserialize;
use crate::config::Config;
use crate::utils;

#[derive(Deserialize)]
pub struct ShortenUrlBody {
    pub url: String,
}

pub async fn redirect(path: web::Path<String>) -> HttpResponse {
    let (url_path) = path.into_inner();
    let Config { db, .. } = Config::new_connection().await.expect("Failed to connect to database");
    let db_connection = db.connect().expect("Failed to connect to database");
    let mut url_rows = db_connection
        .query(
            "SELECT url FROM urls WHERE path = (?1)",
            vec![url_path],
        )
        .await
        .expect("Failed to get url from database");
    let url_data = url_rows.next().await.expect("Failed to get url data");
    match url_data {
        None => {
            HttpResponse::NotFound()
                .body("Url not found")
        }
        Some(url) => {
            let redirect_to_url: String = url.get(0).expect("Failed to get url from row");
            HttpResponse::MovedPermanently()
                .insert_header((
                    actix_web::http::header::LOCATION,
                    redirect_to_url
                ))
                .finish()
        }
    }
}

pub async fn create_shorten_url(body: web::Json<ShortenUrlBody>) -> impl Responder {
    info!("Creating new short url: {:#?}", body.url);
    let Config { db, .. } = Config::new_connection().await.expect("Failed to connect to database");
    let db_connection = db.connect().expect("Failed to connect to database");
    db_connection
        .query(
            "INSERT INTO urls (url) VALUES (?1) RETURNING id",
            vec![body.url.clone()],
        )
        .await
        .expect("Failed to insert new url");
    let row_id = db_connection.last_insert_rowid();
    let short_url_path = utils::convert_to_id(row_id);
    println!("new row id is: {row_id} path {}", short_url_path);
    // todo: generate a new path for the short url using its id
    db_connection.query(
        "update urls set path=(?1) where id = (?2)",
        vec![short_url_path, row_id.to_string()],
    )
        .await
        .expect("Failed to insert new url");

    "Hello, world!"
}
