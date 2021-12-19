use crate::db::auth::User;
use crate::db::Database;
use crate::db::{auth::Identity, messages::Message};
use actix_web::cookie::Cookie;
use actix_web::web;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

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
                    is_own_profile: &identity.user.id == &user.id,
                    user: identity.user,
                    errors: vec![],
                    messages: Some(messages),
                    home: false,
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

#[derive(Serialize, Deserialize)]
pub struct BioInfo {
    content: String,
}

pub async fn post(
    req: HttpRequest,
    bio: web::Form<BioInfo>,
    database: Data<Database>,
) -> impl Responder {
    if let Ok(Some(identity)) = Identity::from_request(&req, &database) {
        match identity.user.update(&bio.content, &database) {
            Ok(_) => HttpResponse::Found()
                .header("location", format!("/user/{}", identity.user.id))
                .finish(),
            Err(e) => HttpResponse::Found()
                .header("location", format!("/user/{}", identity.user.id))
                .cookie(Cookie::build("error", format!("Failed to update bio: {}", e)).finish())
                .finish(),
        }
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}
