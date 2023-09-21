use std::env;
use std::future::Future;
use axum::{Router, handler::get, handler::post, Json};
use axum::extract::{extractor_middleware, Path, Query};
use axum::handler::OnMethod;
use axum::routing::{EmptyRouter, Route};
use dotenv::dotenv;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ObjectList;
use log::{info, warn};

mod api {
    include!("./api/router.rs");
}


mod mysql;
mod kube_cli;

#[tokio::main]
async fn main() {

    //connect to mysql
    let _ = mysql::init_db_pool().await;
    // initialize data with database
    mysql::init_db_table().await;
    mysql::init_db_data().await;
    //connect to kubernetes
    let _ = kube_cli::init_kube_cli().await;

    // logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Info
        .init();

    let app = Router::new()
        //login
        .route("/auth/login", post(api::login))
        //get kubernetes namespaces
        .route("/kube/namespaces", get(
            move |headers| api::namespaces(headers)))
        //get pod list of specific namespace
        .route("/kube/:namespace/pods", get(
            move |path, headers| api::pods(path, headers)))
        //create pod in specific namespace
        .route("/kube/:namespace/pod/create", post(
            move |path, body, headers| api::pod_create(path, body, headers)))
        //get logs of specific pod of specific namespace
        .route("/kube/:namespace/pod/logs",
               get(
                   move |path, body, headers| api::pod_logs(path, body, headers)))
        //get information of specific pod of specific namespace
        .route("/kube/:namespace/pod",
               get(
                   move |path, body, headers| api::pod_info(path, body, headers)));

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


