use std::fmt::Error;
use chrono::{Duration, Local, NaiveDateTime};
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;

use log::error;
use crate::api::dao::entity::User;
use crate::mysql::get_mysql;

pub(crate) mod entity {
    include!("../entity/user.rs");
}


pub async fn query_user_by_account(req: entity::AuthReq) -> Result<User, Error> {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query_as::<_, User>("select * from user where account=? and password=?")
        .bind(req.account)
        .bind(req.password)
        .fetch_one(&mut conn)
        .await.unwrap();

    Ok(row)
}

pub async fn query_user_by_token(token: String) -> Result<User, sqlx::Error> {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query_as::<_, User>("select * from user where token=?")
        .bind(token)
        .fetch_one(&mut conn)
        .await;

    row
}


pub async fn update_token(req: User, token: String) -> Option<sqlx::Error> {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query("update user set token = ?,token_expire=? where id = ?")
        .bind(token)
        .bind(Local::now() + Duration::days(3))
        .bind(req.id)
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
