use super::Templates;
use crate::db::auth::Identity;
use crate::db::Database;
use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use handlebars::Handlebars;
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
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {}

    HttpResponse::Found().header("location", "/").finish()
}
