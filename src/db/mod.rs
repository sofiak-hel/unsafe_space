use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

pub mod auth;
pub mod messages;

use crate::config::Config;
use crate::Result;
use messages::Message;

static INIT_SQL: &str = include_str!("sql/init.sql");
static DROP_SQL: &str = include_str!("sql/drop.sql");

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    config: Config,
}

impl Database {
    pub fn new(config: Config) -> Database {
        Database {
            conn: Arc::new(Mutex::new(Connection::open("db.sqlite3").unwrap())),
            config,
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(INIT_SQL)?;
        Ok(())
    }

    pub fn drop(&mut self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(DROP_SQL)?;
        Ok(())
    }

    pub fn find_user(&self, user_id: u32) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(
            &format!(
                "SELECT username FROM Users 
                    WHERE id={}",
                user_id
            ),
            [],
            |row| row.get(0),
        )?)
    }

    pub fn login(&self, username: &String, password: &String) -> Result<(u32, String)> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(
            &format!(
                "SELECT id, username FROM Users 
                    WHERE username='{}' AND password='{}'",
                username, password
            ),
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?)
    }

    pub fn register(&self, username: &String, password: &String) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            &format!(
                "INSERT INTO users (username, password) 
                    VALUES ('{}', '{}')",
                username, password
            ),
            [],
        )?;
        Ok(())
    }

    pub fn find_session(&self, session_id: &String) -> Result<(u32, u32, Duration)> {
        let conn = self.conn.lock().unwrap();
        let (id, user, expires) = conn.query_row(
            &format!(
                "SELECT id, user, expires FROM Sessions
                    WHERE id={}",
                session_id
            ),
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        let expires = Duration::from_secs(expires);
        Ok((id, user, expires))
    }

    pub fn create_session(&self, user_id: u32) -> Result<(i64, Duration)> {
        let conn = self.conn.lock().unwrap();
        let expires = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?
            + Duration::new(self.config.session_exp, 0);
        conn.execute(
            &format!(
                "INSERT INTO Sessions (user, expires) VALUES ('{}', {})",
                user_id,
                expires.as_secs()
            ),
            [],
        )?;
        Ok((conn.last_insert_rowid(), expires))
    }

    pub fn delete_session(&self, session_id: String) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(&format!("DELETE FROM Sessions where id={}", session_id), [])?;
        Ok(())
    }

    pub fn create_message(&self, user_id: u32, message: String) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        conn.execute(
            &format!(
                "INSERT INTO Messages (user, content, timestamp) VALUES ({}, '{}', {})",
                user_id, message, timestamp
            ),
            [],
        )?;
        Ok(())
    }

    pub fn search_messages(
        &self,
        user_id: Option<u32>,
        message_id: Option<u32>,
    ) -> Result<Vec<Message>> {
        let conn = self.conn.lock().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let mut conditions = Vec::new();
        if let Some(user_id) = user_id {
            conditions.push(format!("user='{}'", user_id));
        }
        if let Some(message_id) = message_id {
            conditions.push(format!("id='{}'", message_id));
        }

        let clause = if conditions.len() > 0 {
            "WHERE ".to_string() + &conditions.join(" AND ")
        } else {
            String::new()
        };

        let mut statement = conn.prepare(&format!(
            "SELECT * FROM Messages as M LEFT JOIN Users AS U ON U.id=M.id {} ORDER BY timestamp",
            clause
        ))?;

        let mut rows = statement.query([])?;

        let mut messages = Vec::new();
        while let Some(row) = rows.next()? {
            messages.push(Message::from_row(&row)?)
        }

        dbg!(&messages);

        Ok(messages)
    }
}
