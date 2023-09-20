use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,

    pub account: String,

    pub password: String,

    pub token: Option<String>,

    #[serde(rename = "tokenExpire")]
    pub token_expire: Option<DateTime<Local>>,

    #[serde(rename = "createTime")]
    pub create_time: DateTime<Local>,
}


#[derive(Serialize, Deserialize)]
pub struct AuthReq {
    pub account: String,

    pub password: String,
}


#[derive(Serialize, Deserialize)]
pub struct AuthResp {
    pub token: String,
}
