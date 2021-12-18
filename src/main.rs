use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod auth;
mod db;
mod error;
mod pages;

use db::Database;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db = Database::new();
    db.initialize().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pages::create_handlebars())
            .data(db.clone())
            .configure(pages::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
