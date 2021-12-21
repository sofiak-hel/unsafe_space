use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

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
    match initialize() {
        Ok((config, db, mimetypes)) => {
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
        }
        Err(e) => {
            log::error!("{}", e);
            log::info!("Shutting down...");
            Ok(())
        }
    }
}

fn initialize() -> Result<(Config, Database, MimeTypes)> {
    let config = if let Ok(c) = Config::from_file("config.toml") {
        c
    } else {
        println!("Failed to load config.toml. Defaulting to default config");
        Config::default()
    };

    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    let mimetypes = match MimeTypes::from_file(&config.mimetypes_path) {
        Ok(m) => m,
        Err(e) => {
            log::error!("{}", e);
            log::warn!(
                "Failed to read mimetypes from {:?}, using default mimetypes",
                &config.mimetypes_path
            );
            MimeTypes::default()
        }
    };

    let mut db = Database::new(config.clone());
    if config.force_recreate_db {
        if let Err(e) = db.drop() {
            log::warn!("Could not drop db: {}", e);
        }
    }
    db.initialize()?;

    Ok((config, db, mimetypes))
}
