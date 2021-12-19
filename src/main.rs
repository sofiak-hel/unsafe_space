use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod auth;
mod config;
mod db;
mod error;
mod pages;

use config::Config;
use db::Database;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::default();

    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    let mut db = Database::new(config.clone());
    if let Err(e) = db.drop() {
        log::warn!("Could not drop db: {}", e);
    }
    if let Err(e) = db.initialize() {
        log::error!("Failed to initialize db: {}", e);
        log::info!("Shutting down...");
        Ok(())
    } else {
        let logging_template = config.logging_template;
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new(&logging_template))
                .data(pages::create_handlebars())
                .data(db.clone())
                .configure(pages::config)
        })
        .bind((config.host.clone(), config.port))?
        .run()
        .await
    }
}
