use crate::db::Database;
use crate::error::USpaceError;
use crate::Result;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::HttpResponseBuilder;
use actix_web::{HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use time;

#[derive(Debug)]
pub struct Identity {
    pub session_id: String,
    pub user: User,
}

impl Identity {
    pub fn from_request(req: &HttpRequest, database: &Database) -> Result<Option<Identity>> {
        if let Some(cookie) = req.cookie("session_id") {
            let (session_id, user_id, expires) =
                database.find_session(&cookie.value().to_owned())?;

            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            if now.lt(&expires) {
                let (username, bio) = database.find_user(user_id)?;
                Ok(Some(Identity {
                    session_id: session_id.to_string(),
                    user: User {
                        id: user_id,
                        username,
                        bio,
                    },
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
        if let Some(user) = User::from_login(username, password, database) {
            let (session_id, expires) = database.create_session(user.id)?;
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub bio: Option<String>,
}

impl User {
    pub fn from_login(username: &String, password: &String, database: &Database) -> Option<User> {
        if let Ok((id, username, bio)) = database.login(username, password) {
            Some(User { id, username, bio })
        } else {
            None
        }
    }

    pub fn create(username: &String, password: &String, database: &Database) -> Result<()> {
        database.register(username, password)
    }

    pub fn get_user(user_id: u32, database: &Database) -> Result<User> {
        let (username, bio) = database.find_user(user_id)?;
        Ok(User {
            id: user_id,
            username,
            bio,
        })
    }

    pub fn update(&self, bio: &String, database: &Database) -> Result<()> {
        database.update_user(self.id, &bio)
    }
}
