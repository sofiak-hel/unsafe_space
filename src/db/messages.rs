use chrono::{DateTime, Local, TimeZone};
use rusqlite::Row;
use serde::{Serialize, Serializer};

use super::{auth::User, Database};
use crate::{error::USpaceError, Result};
use std::result::Result as OriginalResult;

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    id: u32,
    sender: User,
    content: String,
    #[serde(serialize_with = "timestamp_serialize")]
    timestamp: DateTime<Local>,
}

fn timestamp_serialize<S>(
    timestamp: &DateTime<Local>,
    serializer: S,
) -> OriginalResult<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(timestamp.format("%d.%m.%Y %H:%M:%S ").to_string().as_str())
}

impl Message {
    pub fn from_row(row: &Row) -> Result<Message> {
        Ok(Message {
            id: row.get("mid")?,
            sender: User {
                id: row.get("uid")?,
                username: row.get("username")?,
                bio: row.get("bio")?,
            },
            content: row.get("content")?,
            timestamp: Local.timestamp(row.get("timestamp")?, 0),
        })
    }

    pub fn send_message(sender: &User, content: &String, database: &Database) -> Result<()> {
        if content.len() == 0 {
            Err(USpaceError::SendMessageError(
                "Empty message not allowed.".to_owned(),
            ))?
        }
        database.create_message(sender.id, content)?;
        Ok(())
    }

    pub fn all_messages(database: &Database) -> Result<Vec<Message>> {
        database.search_messages(None, None)
    }

    pub fn get_message(database: &Database, message_id: u32) -> Result<Message> {
        if let Some(message) = database.search_messages(None, Some(message_id))?.first() {
            Ok(message.clone())
        } else {
            Err(USpaceError::FetchError("Message not found".to_string()))?
        }
    }

    pub fn by_user(database: &Database, user_id: u32) -> Result<Vec<Message>> {
        database.search_messages(Some(user_id), None)
    }

    pub fn delete(&self, user_id: u32, database: &Database) -> Result<()> {
        database.delete_message(self.id, user_id)
    }
}
