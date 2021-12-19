use std::time::Duration;

use rusqlite::Row;
use serde::{Deserialize, Serialize, Serializer};

use super::{auth::User, Database};
use crate::Result;
use std::result::Result as OriginalResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: u32,
    sender: User,
    content: String,
    #[serde(serialize_with = "timestamp_serialize")]
    timestamp: Duration,
}

fn timestamp_serialize<S>(duration: &Duration, serializer: S) -> OriginalResult<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime = chrono::NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0);
    serializer.serialize_str(datetime.format("%d.%m.%Y %H:%M:%S ").to_string().as_str())
}

impl Message {
    pub fn from_row(row: &Row) -> Result<Message> {
        Ok(Message {
            id: row.get(0)?,
            sender: User {
                id: row.get(1)?,
                username: row.get(5)?,
            },
            content: row.get(2)?,
            timestamp: Duration::from_secs(row.get(3)?),
        })
    }

    pub fn send_message(sender: User, content: String, database: &Database) -> Result<()> {
        database.create_message(sender.id, content)?;
        Ok(())
    }

    pub fn all_messages(database: &Database) -> Result<Vec<Message>> {
        database.search_messages(None, None)
    }
}
