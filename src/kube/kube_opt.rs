use std::convert::TryFrom;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use dotenv::dotenv;
use kube::{api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt}, runtime::wait::{await_condition, conditions::is_pod_running}, Client, client, config, Config, Error};
use kube::api::{LogParams, ObjectList};
use kube::config::{Kubeconfig, KubeconfigError, KubeConfigOptions};
use log::{error, info};
use std::{env, process};
use std::io::BufRead;
use axum::Json;
use crate::controller::entity::PodReq;
use crate::kube_cli::get_kube_cli;
use futures::{StreamExt, TryStreamExt, AsyncBufReadExt};

pub(crate) mod entity {
    include!("../entity/kube_req.rs");
}


pub async fn pod_list(namespace: String) -> ObjectList<Pod> {
    // dotenv().ok();
    // let kube_config = env::var("KUBE_CONFIG").unwrap_or(String::from("~/.kube/config"));
    // let kube_cfg = Kubeconfig::read_from(kube_config).unwrap();
    // let kube_config_option = KubeConfigOptions::default();
    // let mut config = Config::from_custom_kubeconfig(kube_cfg, &kube_config_option).await.unwrap();
    // config.accept_invalid_certs = true;
    // let client = Client::try_from(config).unwrap();
    let client = get_kube_cli();

    // Manage pods
    // let pods: Api<Pod> = Api::default_namespaced(*client);

    let pods: Api<Pod> = if let Some(c) = client {
        let _cli = c.clone();
        Api::namespaced(_cli, &namespace)
    } else {
        panic!("kube client error")
    };

    info!("Get pod list");
    let pod_list = pods.list(&ListParams::default()).await.unwrap();
    return pod_list;
}

pub async fn pod_create(req: PodReq) -> Option<Pod> {
    let client = get_kube_cli();

    // Manage pods
    // let pods: Api<Pod> = Api::default_namespaced(*client);

    let pods: Api<Pod> = if let Some(c) = client {
        let _cli = c.clone();
        Api::namespaced(_cli, &req.namespace.unwrap_or(String::from("default")))
    } else {
        panic!("kube client error")
    };

    info!("create Pod");
    let p: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": "resnet" },
        "spec": {
            "containers": [{
              "name": "resnet",
              "image": "bitnami/tensorflow-serving:latest"
            }],
        }
    })).unwrap();

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created {}", name);
            return Some(o);
        }
        // Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => { error!("{:?}",e.to_string()); }                        // any other case is probably bad
    }

    // Watch it phase for a few seconds
    let establish = await_condition(pods.clone(), "resnet", is_pod_running()).await.unwrap();
    // let result = tokio::time::timeout(std::time::Duration::from_secs(60), establish).await?;
    return establish;
}

pub async fn get_logs(req: PodReq) -> Vec<String> {
    let client = get_kube_cli();

    // Manage pods
    // let pods: Api<Pod> = Api::default_namespaced(*client);

    let pods: Api<Pod> = if let Some(c) = client {
        let _cli = c.clone();
        Api::namespaced(_cli, &req.namespace.unwrap_or(String::from("default")))
    } else {
        panic!("kube client error")
    };

    let all_logs = pods.logs(&req.pod_name.unwrap(), &Default::default()).await.unwrap();

    let lines = all_logs.split("\n").map(|s| s.to_string())
        .filter(|s| !s.is_empty()).collect();
    // output.append(line.to_string());

    return lines;
}