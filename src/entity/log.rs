use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Log {
    pub id: i64,

    pub hash_code: u64,

    pub app: String,

    pub pod: String,

    pub content: String,

    #[serde(rename = "createTime")]
    pub create_time: DateTime<Local>,
}