use std::time::SystemTime;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "userCode")]
    pub user_code: String,

    #[serde(rename = "nickName")]
    pub nick_name: String,

    pub password: String,

    #[serde(rename = "createTime")]
    pub create_time: NaiveDateTime,
}