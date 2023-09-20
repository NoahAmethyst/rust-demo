use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, Error, MySqlPool};
use std::{env, process};
use log::{error, info, log};
use once_cell::sync::OnceCell;
use sqlx::mysql::MySqlConnectOptions;
use tokio::fs;

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();

// connect to mysql
pub async fn init_db_pool() -> Result<(), Error> {
    //When fetching env, you need to use dotenv. Otherwise, it is systematic.
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let _db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_port = _db_port.parse::<u16>().unwrap();
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let conn_opt = MySqlConnectOptions::new()
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
        //Todo why info not working
        Ok(pool) => {
            info!("Connection to the database[{:?}] success.",db_host);
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database[{:?}]: {:?}",db_host,err);
            // println!("🔥 Failed to connect to the database: {:?}", err);
            process::exit(1);
        }
    };
    assert!(MYSQL_POOL.set(pool).is_ok());
    Ok(())
}

// Read immigration file to initialize data of database.
pub async fn init_db_table() {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    // read sql from file
    let sql_file = "./resource/immigration.sql";
    let sql_content = fs::read_to_string(sql_file).await;
    match sql_content {
        Ok(sql) => {
            let result = sqlx::query(&*sql)
                .execute(&*pool).await;

            match result {
                Ok(affected) => {
                    if affected.rows_affected() > 0 {
                        println!("initialize tables success");
                    } else {
                        println!("no need to initialize")
                    }
                }

                Err(err) => {
                    println!("initialize data failed:{:?}", err)
                }
            }
        }

        Err(err) => {
            println!("initialize tables failed:{:?}", err);
        }
    }
}


// Read immigration file to initialize data of database.
pub async fn init_db_data() {
    let pool = get_mysql().expect("Failed to acquire connection pool");

    // read sql from file
    let sql_file = "./resource/data.sql";
    let sql_content = fs::read_to_string(sql_file).await;
    match sql_content {
        Ok(sql) => {
            let result = sqlx::query(&*sql)
                .execute(&*pool).await;

            match result {
                Ok(affected) => {
                    if affected.rows_affected() > 0 {
                        println!("initialize data success");
                    } else {
                        println!("no need to initialize")
                    }
                }

                Err(err) => {
                    println!("initialize data failed:{:?}", err)
                }
            }
        }

        Err(err) => {
            println!("initialize data failed:{:?}", err);
        }
    }
}



// get database
pub fn get_mysql() -> Option<&'static MySqlPool> {
    MYSQL_POOL.get()
}