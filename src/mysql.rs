use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, Error, MySqlPool};
use std::{env, process};
use once_cell::sync::OnceCell;

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();

//建立mysql连接
pub async fn init_db_pool() -> Result<(), Error> {
    //在取env时需要使用dotenv  要不取的是系统的
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            process::exit(1);
        }
    };
    assert!(MYSQL_POOL.set(pool).is_ok());
    Ok(())
}

//获取数据库
pub fn get_pool() -> Option<&'static MySqlPool> {
    MYSQL_POOL.get()
}