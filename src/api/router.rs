use std::{result, time};
use std::future::Future;
use ::kube::api::ObjectList;
use ::kube::config::KubeconfigError;
use ::kube::Error;
use axum::extract::{Path, Query, Json, Form};
use axum::http::{HeaderMap, HeaderValue};
use k8s_openapi::api::core::v1::{Namespace, Pod};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON;
use log::{error, info};
use crate::api::kube::entity::PodReq;
use crate::api::service::common_entity::Resp;
use crate::api::service::user_entity::{AuthReq, AuthResp};


pub(crate) mod kube {
    include!("../kube/kube_opt.rs");
}

mod service {
    include!("../service/auth.rs");
}

// which calls one of these handlers
pub async fn login(Form(req): Form<AuthReq>) -> Json<Resp<AuthResp>> {
    let result = service::login(req).await;
    return Json(result);
}

// get kubernetes pod list
pub async fn namespaces(headers: HeaderMap) -> Json<Resp<ObjectList<Namespace>>> {
    let mut resp = Resp {
        data: None,
        message: None,
    };

    let token = headers.get("Authorization").unwrap().to_str();
    let checked = service::auth(token.unwrap().to_string()).await;
    if !checked {
        resp.message = Some(String::from("unauthorized,please login first."));
        return Json(resp);
    }
    let result = kube::namespaces().await;

    resp.data = Some(result);

    return Json(resp);
}

// get kubernetes pod list
pub async fn pods(Path(namespace): Path<String>, headers: HeaderMap) -> Json<Resp<ObjectList<Pod>>> {
    let mut resp = Resp {
        data: None,
        message: None,
    };

    let token = headers.get("Authorization").unwrap().to_str();
    let checked = service::auth(token.unwrap().to_string()).await;
    if !checked {
        resp.message = Some(String::from("unauthorized,please login first."));
        return Json(resp);
    }
    let _namespace = namespace.clone();
    let result = kube::pod_list(namespace).await;

    resp.data = Some(result);

    return Json(resp);
}

// create resnet pod
pub async fn pod_create(Path(namespace): Path<String>, mut req: Json<PodReq>, headers: HeaderMap) -> Json<Resp<Option<Pod>>> {
    let _namespace = namespace.clone();
    req.0.namespace = Some(namespace);
    let mut resp = Resp {
        data: None,
        message: None,
    };

    let token = headers.get("Authorization").unwrap().to_str();
    let checked = service::auth(token.unwrap().to_string()).await;
    if !checked {
        resp.message = Some(String::from("unauthorized,please login first."));
        return Json(resp);
    }

    let result = kube::pod_create(req.0).await;

    resp.data = Some(result);

    return Json(resp);
}

pub async fn pod_logs(Path(namespace): Path<String>, mut req: Query<PodReq>, headers: HeaderMap) -> Json<Resp<Vec<String>>> {
    let mut resp = Resp {
        data: None,
        message: None,
    };

    let token = headers.get("Authorization").unwrap().to_str();
    let checked = service::auth(token.unwrap().to_string()).await;
    if !checked {
        resp.message = Some(String::from("unauthorized,please login first."));
        return Json(resp);
    }
    let _namespace = namespace.clone();
    req.0.namespace = Some(namespace);
    let result = kube::pod_logs(req.0).await;
    match result {
        Ok(logs) => {
            resp.data = Some(logs);
        }

        Err(err) => {
            resp.message = Some(err.to_string())
        }
    }

    return Json(resp);
}

pub async fn pod_info(Path(namespace): Path<String>, mut req: Query<PodReq>, headers: HeaderMap) -> Json<Resp<Pod>> {
    let mut resp = Resp {
        data: None,
        message: None,
    };

    let token = headers.get("Authorization").unwrap().to_str();
    let checked = service::auth(token.unwrap().to_string()).await;
    if !checked {
        resp.message = Some(String::from("unauthorized,please login first."));
        return Json(resp);
    }
    let _namespace = namespace.clone();
    req.0.namespace = Some(namespace);
    let result = kube::pod_info(req.0).await;
    match result {
        Ok(info) => {
            resp.data = Some(info);
        }
        Err(err) => {
            resp.message = Some(err.to_string())
        }
    }

    return Json(resp);
}

