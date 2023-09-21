use std::fmt::Error;
use chrono::{Duration, Local, NaiveDateTime};
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;

use log::error;
use crate::api::kube::log_dao::log_entity::Log;
use crate::mysql::get_mysql;

pub(crate) mod log_entity {
    include!("../entity/log.rs");
}

pub async fn query_log_by_app(app: String) -> Result<Vec<Log>, sqlx::Error> {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    let mut conn = pool.acquire().await.unwrap();

    let rows = sqlx::query_as::<_, Log>("select * from logs where app=? order by id")
        .bind(app)
        .fetch_all(&mut conn)
        .await;

    rows
}

pub async fn insert_log(log: Log) -> Option<sqlx::Error> {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query("INSERT INTO logs (hash_code,app,pod,content) VALUES (?,?,?,?)")
        .bind(log.hash_code)
        .bind(log.app)
        .bind(log.pod)
        .bind(log.content)
        .execute(&mut conn)
        .await;

    return match row {
        Ok(_) => {
            None
        }

        Err(err) => {
            Some(err)
        }
    };
}