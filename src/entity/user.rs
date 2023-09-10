use std::time::SystemTime;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[serde(rename = "id")]
    #[serde(alias = "id")]
    pub Id: i64,
    #[serde(rename = "user_code")]
    #[serde(alias = "userCode")]
    pub UserCode: String,
    #[serde(rename = "nick_name")]
    #[serde(alias = "nickName")]
    pub NickName: String,
    #[serde(rename = "password")]
    #[serde(alias = "passWord")]
    pub PassWord: String,
    #[serde(rename = "create_time")]
    #[serde(alias = "createTime")]
    pub CreateTime: NaiveDateTime,
}