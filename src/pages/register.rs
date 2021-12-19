use super::Pages;
use crate::auth::Identity;
use crate::db::Database;
use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn get(
    req: HttpRequest,
    handlebars: Data<Handlebars<'_>>,
    database: Data<Database>,
) -> impl Responder {
    if let Ok(Some(_)) = Identity::from_request(&req, &database) {
        HttpResponse::Found().header("location", "/").finish()
    } else {
        let page = handlebars
            .render(&Pages::REGISTER.to_string(), &serde_json::json!({}))
            .unwrap();

        HttpResponse::Ok().body(page)
    }
}

pub async fn post(
    json: Form<LoginInfo>,
    database: Data<Database>,
    handlebars: Data<Handlebars<'_>>,
) -> impl Responder {
    match Identity::register(&json.username, &json.password, &database) {
        Ok(_) => HttpResponse::Found().header("location", "/login").finish(),
        Err(_) => {
            let page = handlebars
                .render(
                    &Pages::REGISTER.to_string(),
                    &serde_json::json!({
                        "error": "User already exists"
                    }),
                )
                .unwrap();

            HttpResponse::Ok().body(page)
        }
    }
}
