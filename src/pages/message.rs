use crate::db::Database;
use crate::db::{auth::Identity, messages::Message};
use actix_web::cookie::Cookie;
use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

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
