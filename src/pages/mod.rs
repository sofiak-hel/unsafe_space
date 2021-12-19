mod index;
mod login;
mod logout;
mod register;

use actix_web::web;
use handlebars::Handlebars;

static INDEX_PAGE: &str = include_str!("html/index.html");
static LOGIN_PAGE: &str = include_str!("html/login.html");
static REGISTER_PAGE: &str = include_str!("html/register.html");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index::get)))
        .service(
            web::resource("/login")
                .route(web::get().to(login::get))
                .route(web::post().to(login::post)),
        )
        .service(web::resource("/logout").route(web::get().to(logout::get)))
        .service(
            web::resource("/register")
                .route(web::get().to(register::get))
                .route(web::post().to(register::post)),
        );
}

#[derive(Debug)]
pub enum Pages {
    INDEX,
    LOGIN,
    REGISTER,
}

impl std::fmt::Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
        .register_template_string(&Pages::REGISTER.to_string(), REGISTER_PAGE)
        .unwrap();
    handlebars
}
