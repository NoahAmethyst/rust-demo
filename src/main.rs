use std::env;
use axum::{Router,
           handler::get, handler::post};
use dotenv::dotenv;
use log::{info, warn};

mod controller {
    include!("./controller/router.rs");
}

mod mysql;
mod kube_cli;

#[tokio::main]
async fn main() {

    //connect to mysql
    let _ = mysql::init_db_pool().await;
    //connect to kubernetes
    let _ = kube_cli::init_kube_cli().await;
    // logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Info
        .init();
    // our router
    let app = Router::new()
        .route("/user", get(controller::user))
        .route("/kube/:namespace/pods", get(
            move |path| controller::pods(path)))//get pod list of specific namespace
        .route("/kube/:namespace/pod/create", post(
            move |path, body| controller::pod_create(path, body)))//create pod in specific namespace
        .route("/kube/:namespace/pod/logs",
               get(
                   move |path, body| controller::pod_logs(path, body)))//get logs of specific pod of specific namespace
        .route("/kube/:namespace/pod",
               get(
                   move |path, body| controller::pod_info(path, body)));//get information of specific pod of specific namespace


    dotenv().ok();
    // run it with hyper on localhost:8080
    let server_port = env::var("SERVER_PORT").unwrap_or(String::from("8080"));
    let mut addr = String::from("0.0.0.0:");
    addr.push_str(&server_port);
    info!("server start at {}",addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

