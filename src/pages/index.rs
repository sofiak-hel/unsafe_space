use super::Pages;
use crate::auth::Identity;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;

pub async fn get(req: HttpRequest, handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    if let Some(identity) = Identity::from_request(req) {
        let index = handlebars
            .render(
                &Pages::INDEX.to_string(),
                &serde_json::json!({ "session": identity.session_id, "username": identity.username }),
            )
            .unwrap();

        HttpResponse::Ok().body(index)
    } else {
        HttpResponse::Found().header("location", "/login").finish()
    }
}
