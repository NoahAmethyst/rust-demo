use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, Error, MySqlPool};
use std::{env, process};
use once_cell::sync::OnceCell;
use sqlx::mysql::MySqlConnectOptions;

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();

//建立mysql连接
pub async fn init_db_pool() -> Result<(), Error> {
    //在取env时需要使用dotenv  要不取的是系统的
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let _db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_port=_db_port.parse::<u16>().unwrap();
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let conn_opt=MySqlConnectOptions::new()
        .username(&db_username)
        .password(&db_password)
        .host(&db_host)
        .database(&db_name)
        .port(db_port);

    let pool = match MySqlPoolOptions::new()
        .max_connections(100)
        .connect_with(conn_opt)
        // .connect(&database_url)
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