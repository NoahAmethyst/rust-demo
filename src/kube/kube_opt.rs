use std::convert::TryFrom;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use dotenv::dotenv;
use kube::{api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt}, runtime::wait::{await_condition, conditions::is_pod_running}, Client, client, config, Config, Error};
use kube::api::ObjectList;
use kube::config::{Kubeconfig, KubeconfigError, KubeConfigOptions};
use log::info;
use std::{env, process};

pub async fn pod_list() -> ObjectList<Pod> {
    dotenv().ok();
    let kube_config = env::var("KUBE_CONFIG").unwrap_or(String::from("~/.kube/config"));
    let kube_cfg = Kubeconfig::read_from(kube_config).unwrap();
    let kube_config_option = KubeConfigOptions::default();
    let mut config = Config::from_custom_kubeconfig(kube_cfg, &kube_config_option).await.unwrap();
    config.accept_invalid_certs = true;
    let client = Client::try_from(config).unwrap();

    // Manage pods
    let pods: Api<Pod> = Api::default_namespaced(client);

    info!("Get Pod qq-bot");
    let pod_list = pods.list(&ListParams::default()).await.unwrap();
    return  pod_list
}