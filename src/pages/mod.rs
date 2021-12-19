mod index;
mod login;
mod logout;

use actix_web::web;
use handlebars::Handlebars;

static INDEX_PAGE: &str = include_str!("html/index.html");
static LOGIN_PAGE: &str = include_str!("html/login.html");

#[derive(Debug)]
pub enum Pages {
    INDEX,
    LOGIN,
}

impl std::fmt::Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index::get)))
        .service(
            web::resource("/login")
                .route(web::get().to(login::get))
                .route(web::post().to(login::post)),
        )
        .service(web::resource("/logout").route(web::get().to(logout::get)));
}

pub fn create_handlebars<'reg>() -> Handlebars<'reg> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(&Pages::INDEX.to_string(), INDEX_PAGE)
        .unwrap();
    handlebars
        .register_template_string(&Pages::LOGIN.to_string(), LOGIN_PAGE)
        .unwrap();
    handlebars
}
