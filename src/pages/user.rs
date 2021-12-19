use crate::db::auth::User;
use crate::db::Database;
use crate::db::{auth::Identity, messages::Message};
use actix_web::cookie::Cookie;
use actix_web::web;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;

use super::timeline::{self, TimelineData};

pub async fn get(
    req: HttpRequest,
    user_id: web::Path<u32>,
    handlebars: Data<Handlebars<'_>>,
    database: Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        match (
            User::get_user(*user_id, &database),
            Message::by_user(&database, *user_id),
        ) {
            (Ok(user), Ok(messages)) => timeline::render_timeline(
                &req,
                &handlebars,
                &mut TimelineData {
                    user: identity.user,
                    errors: vec![],
                    messages: Some(messages),
                    not_home: true,
                    profile: Some(user),
                },
            ),
            (_, _) => HttpResponse::Found()
                .header("location", "/")
                .cookie(Cookie::build("error", format!("Could not find user.")).finish())
                .finish(),
        }
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}
