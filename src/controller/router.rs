use std::{result, time};
use ::kube::api::ObjectList;
use ::kube::config::KubeconfigError;
use ::kube::Error;
use axum::extract::{Path, Query, Json};
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON;
use log::error;


mod dao {
    include!("../dao/user_db.rs");
}

mod entity {
    include!("../entity/kube_req.rs");
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

// get kubernetes pod list
pub async fn pods(Path(namespace): Path<String>) -> Json<ObjectList<Pod>> {
    let mut _namespace = namespace.clone();
    if _namespace.is_empty() {
        _namespace = String::from("default");
    }
    let result = kube::pod_list(_namespace);
    // match result.await {
    //     Ok(pod_list)=>Json(pod_list),
    //     Err(err)=> {
    //         error!("call kube error:{:?}",err.to_string());
    //         Json(ObjectList{ metadata: Default::default(), items: vec![] })
    //     }
    // }
    return Json(result.await);
}

// create resnet pod
pub async fn pod_create(Path(namespace): Path<String>, mut req: Json<entity::PodReq>) -> Json<Option<Pod>> {
    req.0.namespace = Some(namespace);
    let result = kube::pod_create(req.0);
    return Json(result.await);
}

pub async fn pod_logs(Path(namespace): Path<String>, mut req: Query<entity::PodReq>) -> Json<Vec<String>> {
    req.0.namespace = Some(namespace);
    let result = kube::get_logs(req.0);
    return Json(result.await);
}

