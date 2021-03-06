use crate::db::Database;
use crate::db::{auth::Identity, messages::Message};
use actix_web::cookie::Cookie;
use actix_web::web;
use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use super::timeline::{self, TimelineData};

pub async fn get(
    req: HttpRequest,
    message_id: web::Path<u32>,
    handlebars: Data<Handlebars<'_>>,
    database: Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        match Message::get_message(&database, *message_id) {
            Ok(message) => timeline::render_timeline(
                &req,
                &handlebars,
                &mut TimelineData {
                    user: identity.user,
                    messages: Some(vec![message]),
                    errors: vec![],
                    home: false,
                    profile: None,
                },
            ),
            Err(e) => HttpResponse::Found()
                .header("location", "/")
                .cookie(Cookie::build("error", format!("Message not found: {}", e)).finish())
                .finish(),
        }
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageForm {
    message: String,
}

pub async fn post(
    req: HttpRequest,
    message: Form<MessageForm>,
    database: Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        match Message::send_message(&identity.user, &message.message, &database) {
            Ok(_) => HttpResponse::Found().header("location", "/").finish(),
            Err(e) => HttpResponse::Found()
                .header("location", "/")
                .cookie(Cookie::build("error", format!("Failed to send message: {}", e)).finish())
                .finish(),
        }
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}

pub async fn delete(message_id: web::Path<u32>, database: Data<Database>) -> impl Responder {
    match Message::get_message(&database, *message_id) {
        Ok(message) => {
            message.delete(&database).ok();
            HttpResponse::Found().header("location", "/").finish()
        }
        _ => HttpResponse::Found()
            .header("location", "/")
            .cookie(Cookie::build("error", format!("Message not found")).finish())
            .finish(),
    }
}
