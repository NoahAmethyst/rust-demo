use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use log::error;
use crate::mysql::get_pool;

pub(crate) mod entity {
    include!("../entity/user.rs");
}


pub async fn query_user() -> Result<Vec<entity::User>, sqlx::Error> {
    let pool = get_pool().expect("Failed to acquire connection pool");
    ;

    let mut conn = pool.acquire().await?;

    let  rows = sqlx::query_as::<_, entity::User>("select id,user_code,nick_name,password,create_time from bg_user")
        .fetch_all(&mut conn)
        .await?;

    Ok(rows)
}
