use crate::db::Database;
use crate::Result;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpMessage, HttpRequest};
use time::Duration;

pub struct Identity {
    pub session_id: String,
    pub username: String,
}

impl Identity {
    pub fn from_request(req: HttpRequest, database: &Database) -> Result<Option<Identity>> {
        if let Some(cookie) = req.cookie("session_id") {
            let (session_id, user_id) = database.find_session(&cookie.value().to_owned())?;
            let username = database.find_user(user_id)?;
            Ok(Some(Identity {
                session_id: session_id.to_string(),
                username,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn login(
        username: &String,
        password: &String,
        database: &Database,
    ) -> Result<Option<Cookie<'static>>> {
        let (id, _) = database.login(username, password)?;
        let session_id = database.create_session(id)?;
        Ok(Some(
            Cookie::build("session_id", session_id.to_string())
                .max_age(Duration::minutes(30))
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish(),
        ))
    }
}
