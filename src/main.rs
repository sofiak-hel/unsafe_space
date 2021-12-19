use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod auth;
mod config;
mod db;
mod error;
mod mime;
mod pages;

use config::Config;
use db::Database;
use mime::MimeTypes;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok((config, db, mimetypes)) = initialize() {
        let config_clone = config.clone();
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new(&config_clone.logging_template))
                .data(pages::create_handlebars())
                .data(db.clone())
                .data(config_clone.clone())
                .data(mimetypes.clone())
                .configure(pages::config)
        })
        .bind((config.host.clone(), config.port))?
        .run()
        .await
    } else {
        log::info!("Shutting down...");
        Ok(())
    }
}

fn initialize() -> Result<(Config, Database, MimeTypes)> {
    let config = Config::default();

    let mimetypes = MimeTypes::from(&config.mimetypes_path)?;

    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    let mut db = Database::new(config.clone());
    if let Err(e) = db.drop() {
        log::warn!("Could not drop db: {}", e);
    }
    db.initialize()?;

    Ok((config, db, mimetypes))
}
