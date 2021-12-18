use super::Pages;
use crate::auth::Identity;
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

pub async fn post(json: web::Form<LoginInfo>) -> impl Responder {
    HttpResponse::Found()
        .header("location", "/")
        .cookie(Identity::login(&json.username, &json.password).unwrap())
        .finish()
}
