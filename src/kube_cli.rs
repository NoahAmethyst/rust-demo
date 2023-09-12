use std::convert::TryFrom;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use dotenv::dotenv;
use kube::{api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt}, runtime::wait::{await_condition, conditions::is_pod_running}, Client, client, config, Config, Error};
use kube::api::ObjectList;
use kube::config::{Kubeconfig, KubeconfigError, KubeConfigOptions};
use log::info;
use std::{env, process};
use once_cell::sync::OnceCell;


static KUBE_CLI: OnceCell<Client> = OnceCell::new();


pub async fn init_kube_cli() -> Result<(), Error> {
    dotenv().ok();
    let kube_config = env::var("KUBE_CONFIG").unwrap_or(String::from("~/.kube/config"));
    let kube_cfg = Kubeconfig::read_from(kube_config).unwrap();
    let kube_config_option = KubeConfigOptions::default();
    let mut config = Config::from_custom_kubeconfig(kube_cfg, &kube_config_option).await.unwrap();
    config.accept_invalid_certs = true;
    let client = Client::try_from(config)?;
    // match client {
    //     Ok(client) => {
    //         info!("Init kubernetes client successful!");
    //         client
    //     }
    //     Err(err) => {
    //         println!("Failed to init kubernetes client: {:?}", err);
    //     }
    // };
    assert!(KUBE_CLI.set(client).is_ok());
    Ok(())
}


pub fn get_kube_cli() -> Option<&'static Client> {
    KUBE_CLI.get()
}