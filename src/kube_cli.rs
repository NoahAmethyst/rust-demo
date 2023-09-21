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
    assert!(KUBE_CLI.set(client).is_ok());
    //Todo why info not work
    info!("connect to kubernetes success.");
    println!("connect to kubernetes success.");

    Ok(())
}


pub fn get_kube_cli() -> Option<&'static Client> {
    KUBE_CLI.get()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod kube_cli {
    use std::io::{BufRead, Lines};
    use futures_util::{TryStreamExt, AsyncBufReadExt};
    use k8s_openapi::api::core::v1::Namespace;
    use kube::api::LogParams;
    use log::log;
    use super::*;

    #[actix_rt::test]
    async fn kube_get_all_log() {
        let _ = init_kube_cli().await;
        let cli = get_kube_cli();
        let pods: Api<Pod> = if let Some(c) = cli {
            let _cli = c.clone();
            Api::default_namespaced(_cli)
        } else {
            panic!("kube client error")
        };
        // Get current list of logs
        let lp = LogParams {
            follow: true,
            ..LogParams::default()
        };
        // let mut logs_stream = pods.log_stream("qq-bot-86dbd58985-fxsff", &lp).await.unwrap().lines();
        //
        // // wait for container to finish
        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let all_logs = pods.logs("qq-bot-86dbd58985-fxsff", &Default::default()).await.unwrap();
        println!("{}", all_logs);


        // individual logs may or may not buffer

        // while let Some(line) = logs_stream.try_next().await.unwrap() {
        //     print!("{}", line);
        // }
    }


    #[actix_rt::test]
    async fn kube_get_log_stream() {
        let _ = init_kube_cli().await;
        let cli = get_kube_cli();
        let pods: Api<Pod> = if let Some(c) = cli {
            let _cli = c.clone();
            Api::default_namespaced(_cli)
        } else {
            panic!("kube client error")
        };
        // Get current list of logs
        let lp = LogParams {
            follow: true,
            ..LogParams::default()
        };

        let mut logs_stream;
        let result = pods.log_stream("qq-bot-86dbd58985-dp5h4", &lp);
        match result.await {
            Ok(stream) => { logs_stream = stream.lines() }
            Err(err) => {
                panic!("{:?}", err.to_string())
            }
        }

        // individual logs may or may not buffer
        while let line = logs_stream.try_next().await.unwrap() {
            println!("{:?}", String::from(line.unwrap()));
        }
    }


    #[actix_rt::test]
    async fn kube_namespaces() {
        let _ = init_kube_cli().await;
        let cli = get_kube_cli();
        let namespaces: Api<Namespace> = if let Some(c) = cli {
            let _cli = c.clone();
            Api::all(_cli)
        } else {
            panic!("kube client error")
        };

        let lp = ListParams::default();
        let result = namespaces.list(&lp).await;
        match result {
            Ok(namespaces) => {
                for namespace in namespaces.iter(){
                    println!("{:?}", namespace)
                }

            }

            Err(err) => {
                panic!(err.to_string())
            }
        }
    }
}