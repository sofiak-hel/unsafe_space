use crate::db::Database;
use crate::error::USpaceError;
use crate::Result;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::HttpResponseBuilder;
use actix_web::{HttpMessage, HttpRequest};
use std::time::SystemTime;
use time;

pub struct Identity {
    pub session_id: String,
    pub username: String,
}

impl Identity {
    pub fn from_request(req: &HttpRequest, database: &Database) -> Result<Option<Identity>> {
        if let Some(cookie) = req.cookie("session_id") {
            let (session_id, user_id, expires) =
                database.find_session(&cookie.value().to_owned())?;

            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            if now.lt(&expires) {
                let username = database.find_user(user_id)?;
                Ok(Some(Identity {
                    session_id: session_id.to_string(),
                    username,
                }))
            } else {
                database.delete_session(session_id.to_string()).ok();
                Err(USpaceError::SessionExpired)?
            }
        } else {
            Ok(None)
        }
    }

    pub fn clear_session(
        req: &HttpRequest,
        mut res: HttpResponseBuilder,
        database: &Database,
    ) -> HttpResponseBuilder {
        if let Some(cookie) = req.cookie("session_id") {
            res.del_cookie(&cookie);
            database.delete_session(cookie.value().to_owned()).ok();
        }
        res
    }

    pub fn login(
        username: &String,
        password: &String,
        database: &Database,
    ) -> Result<Option<Cookie<'static>>> {
        if let Ok((id, _)) = database.login(username, password) {
            let (session_id, expires) = database.create_session(id)?;
            log::info!("{} logged in", username);
            Ok(Some(
                Cookie::build("session_id", session_id.to_string())
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .expires(time::OffsetDateTime::from_unix_timestamp(
                        expires.as_secs() as i64
                    ))
                    .finish(),
            ))
        } else {
            Ok(None)
        }
    }

    pub fn register(username: &String, password: &String, database: &Database) -> Result<()> {
        database.register(username, password)
    }
}
