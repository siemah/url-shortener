use actix_web::{App, HttpServer, web};
use crate::routes::{create_shorten_url, redirect};

pub async fn init_router(addr: String) {
    HttpServer::new(move || {
        App::new()
            .route("/{path}", web::get().to(redirect))
            .route("/new", web::post().to(create_shorten_url))
    })
        .bind(&addr)
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Failed to run server");
}