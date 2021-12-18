use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::Result;

static INIT_SQL: &str = include_str!("init.sql");

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            conn: Arc::new(Mutex::new(Connection::open("db.sqlite3").unwrap())),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(INIT_SQL)?;
        Ok(())
    }

    pub fn find_user(&self, user_id: u32) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(
            &format!(
                "select username from Users 
                    where id={}",
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
                "select id, username from Users 
                    where username='{}' and password='{}'",
                username, password
            ),
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?)
    }

    pub fn find_session(&self, session_id: &String) -> Result<(u32, u32)> {
        let conn = self.conn.lock().unwrap();
        Ok(conn.query_row(
            &format!(
                "select id, user from Sessions
                    where id={}",
                session_id
            ),
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?)
    }

    pub fn create_session(&self, user_id: u32) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            &format!("INSERT INTO Sessions (user) VALUES ('{}')", user_id),
            [],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
