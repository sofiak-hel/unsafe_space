use super::Pages;
use crate::auth::Identity;
use crate::db::Database;
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn get(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    let index = handlebars
        .render(&Pages::LOGIN.to_string(), &serde_json::json!({}))
        .unwrap();

    HttpResponse::Ok().body(index)
}

pub async fn post(json: web::Form<LoginInfo>, database: web::Data<Database>) -> impl Responder {
    match Identity::login(&json.username, &json.password, &database) {
        Ok(res) => {
            if let Some(session_cookie) = res {
                HttpResponse::Found()
                    .header("location", "/")
                    .cookie(session_cookie)
                    .finish()
            } else {
                HttpResponse::InternalServerError().body("The hek")
            }
        }
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("Db error")
        }
    }
}
