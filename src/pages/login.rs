use super::Templates;
use crate::db::auth::Identity;
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
            .render(&Templates::Login.to_string(), &serde_json::json!({}))
            .unwrap();

        Identity::clear_session(&req, HttpResponse::Ok(), &database).body(page)
    }
}

pub async fn post(
    json: Form<LoginInfo>,
    database: Data<Database>,
    handlebars: Data<Handlebars<'_>>,
) -> impl Responder {
    match Identity::login(&json.username, &json.password, &database) {
        Ok(res) => {
            if let Some(session_cookie) = res {
                HttpResponse::Found()
                    .header("location", "/")
                    .cookie(session_cookie)
                    .finish()
            } else {
                let page = handlebars
                    .render(
                        &Templates::Login.to_string(),
                        &serde_json::json!({"error": "No such user"}),
                    )
                    .unwrap();

                HttpResponse::Ok().body(page)
            }
        }
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
}
