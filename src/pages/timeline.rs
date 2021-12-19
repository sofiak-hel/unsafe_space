use super::Templates;
use crate::db::auth::{Identity, User};
use crate::db::messages::Message;
use crate::db::Database;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPage {
    pub user: User,
    pub messages: Option<Vec<Message>>,
    pub errors: Vec<String>,
}

pub async fn get(
    req: HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    database: web::Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        let mut errors = Vec::new();
        let messages = match Message::all_messages(&database) {
            Ok(messages) => Some(messages),
            Err(e) => {
                log::error!("{}", e);
                errors.push("Unable to fetch messages:".to_string());
                None
            }
        };

        render_timeline(
            &req,
            &handlebars,
            &mut IndexPage {
                user: identity.user,
                messages: messages,
                errors,
            },
        )
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}

pub fn render_timeline(
    req: &HttpRequest,
    handlebars: &Handlebars<'_>,
    page: &mut IndexPage,
) -> HttpResponse {
    let mut errors = Vec::new();

    let error_cookie = req.cookie("error");
    if let Some(cookie) = &error_cookie {
        errors.push(cookie.value().to_owned());
    }

    page.errors.extend(errors);

    let index = handlebars
        .render(&Templates::Timeline.to_string(), &page)
        .unwrap();

    let mut response = HttpResponse::Ok();
    if let Some(error_cookie) = &error_cookie {
        response.del_cookie(error_cookie);
    }
    response.body(index)
}
