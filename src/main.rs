use axum::{Router,
           handler::get};
use log::{info, warn};

mod controller {
    include!("./controller/router.rs");
}

mod mysql;
mod kube_cli;

#[tokio::main]
async fn main() {

    //连接数据库
    mysql::init_db_pool().await;
    kube_cli::init_kube_cli().await;
    // logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Info
        .init();
    // our router
    let app = Router::new()
        .route("/user", get(controller::user))
        .route("/kube/pods", get(controller::pods))
        .route("/kube/pod/create", get(controller::pod_create));

    // run it with hyper on localhost:3000
    let addr = "0.0.0.0:3000";
    info!("server start at {}",addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

