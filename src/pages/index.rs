use super::Templates;
use crate::db::auth::{Identity, User};
use crate::db::messages::Message;
use crate::db::Database;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct IndexPage {
    user: User,
    messages: Option<Vec<Message>>,
    error: Option<String>,
}

pub async fn get(
    req: HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    database: web::Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        let (messages, error) = match Message::all_messages(&database) {
            Ok(messages) => (Some(messages), None),
            Err(e) => {
                log::error!("{}", e);
                (None, Some("Unable to fetch messages:".to_string()))
            }
        };

        let index = handlebars
            .render(
                &Templates::Index.to_string(),
                &IndexPage {
                    user: identity.user,
                    messages,
                    error,
                },
            )
            .unwrap();

        HttpResponse::Ok().body(index)
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}
