use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpMessage, HttpRequest};
use time::Duration;

pub struct Identity {
    pub session_id: String,
    pub username: String,
}

impl Identity {
    pub fn from_request(req: HttpRequest) -> Option<Identity> {
        if let Some(cookie) = req.cookie("session_id") {
            Some(Identity {
                session_id: cookie.value().to_owned(),
                username: "Otus".to_string(),
            })
        } else {
            None
        }
    }

    pub fn login(_username: &String, _password: &String) -> Option<Cookie<'static>> {
        let session_id = 123.to_string();
        Some(
            Cookie::build("session_id", session_id.to_string())
                .max_age(Duration::minutes(30))
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish(),
        )
    }
}
