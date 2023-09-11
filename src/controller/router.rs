use std::{result, time};
use std::future::Future;
use ::kube::api::ObjectList;
use ::kube::config::KubeconfigError;
use axum::Json;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON;
use log::error;


mod dao {
    include!("../dao/user_db.rs");
}

mod kube {
    include!("../kube/kube_opt.rs");
}

// which calls one of these handlers
pub async fn user() -> Json<Vec<dao::entity::User>> {
    let result = dao::query_user();
    match result.await {
        Ok(users) => Json(users),
        Err(_) => Json(vec![]),  // 返回一个空的用户列表或根据实际情况处理错误
    }
}

pub async fn pods() -> Json<ObjectList<Pod>> {
    let result = kube::pod_list();
    // match result.await {
    //     Ok(pod_list)=>Json(pod_list),
    //     Err(err)=> {
    //         error!("call kube error:{:?}",err.to_string());
    //         Json(ObjectList{ metadata: Default::default(), items: vec![] })
    //     }
    // }
    return Json(result.await)
}

pub async fn post_foo() -> String {
    String::from("post:foo")
}

pub async fn foo_bar() -> String {
    String::from("foo:bar")
}

