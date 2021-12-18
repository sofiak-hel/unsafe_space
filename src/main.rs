use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod auth;
mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(pages::create_handlebars())
            .configure(pages::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
