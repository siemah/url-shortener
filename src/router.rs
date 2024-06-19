use actix_web::{App, HttpServer, web};
use actix_web::dev::{AppService, ServiceFactory};
use crate::routes::{create_shorten_url, index};

pub async fn init_router(addr: String) {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/new", web::post().to(create_shorten_url))
    })
        .bind(&addr)
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Failed to run server");
}