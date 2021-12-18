use super::Pages;
use crate::auth::Identity;
use crate::db::Database;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;

pub async fn get(
    req: HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
    database: web::Data<Database>,
) -> impl Responder {
    match Identity::from_request(req, &database) {
        Ok(res) => {
            if let Some(identity) = res {
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
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("Db error")
        }
    }
}
