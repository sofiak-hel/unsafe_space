use super::Templates;
use crate::db::auth::{Identity, User};
use crate::db::messages::Message;
use crate::db::Database;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPage {
    user: User,
    messages: Option<Vec<Message>>,
    errors: Vec<String>,
}

pub async fn get(
    req: HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    database: web::Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        let mut errors = Vec::new();

        let error_cookie = req.cookie("error");
        if let Some(cookie) = &error_cookie {
            errors.push(cookie.value().to_owned());
        }

        let messages = match Message::all_messages(&database) {
            Ok(messages) => Some(messages),
            Err(e) => {
                log::error!("{}", e);
                errors.push("Unable to fetch messages:".to_string());
                None
            }
        };

        let index = handlebars
            .render(
                &Templates::Index.to_string(),
                &IndexPage {
                    user: identity.user,
                    messages,
                    errors,
                },
            )
            .unwrap();

        let mut response = HttpResponse::Ok();
        if let Some(error_cookie) = &error_cookie {
            response.del_cookie(error_cookie);
        }
        response.body(index)
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}
