mod files;
mod login;
mod logout;
mod message;
mod register;
mod timeline;
mod user;

use actix_web::web;
use handlebars::Handlebars;

static TIMELINE_PAGE: &str = include_str!("html/timeline.html");
static LOGIN_PAGE: &str = include_str!("html/login.html");
static REGISTER_PAGE: &str = include_str!("html/register.html");
static MESSAGE_COMPONENT: &str = include_str!("html/message-component.html");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/static").configure(files::config))
        .service(web::resource("/").route(web::get().to(timeline::get)))
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
        )
        .service(web::resource("/message").route(web::post().to(message::post)))
        .service(web::resource("/message/{id}").route(web::get().to(message::get)))
        .service(web::resource("/user/{id}").route(web::get().to(user::get)))
        .service(web::resource("/user").route(web::post().to(user::post)));
}

#[derive(Debug)]
pub enum Templates {
    Timeline,
    Login,
    Register,
    MessageComponent,
}

impl std::fmt::Display for Templates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn create_handlebars<'reg>() -> Handlebars<'reg> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(&Templates::Timeline.to_string(), TIMELINE_PAGE)
        .unwrap();
    handlebars
        .register_template_string(&Templates::Login.to_string(), LOGIN_PAGE)
        .unwrap();
    handlebars
        .register_template_string(&Templates::Register.to_string(), REGISTER_PAGE)
        .unwrap();
    handlebars
        .register_template_string(&Templates::MessageComponent.to_string(), MESSAGE_COMPONENT)
        .unwrap();
    handlebars
}
