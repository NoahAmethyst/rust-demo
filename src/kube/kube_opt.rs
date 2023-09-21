use std::convert::TryFrom;
use k8s_openapi::api::core::v1::{Namespace, Pod};
use serde_json::json;
use dotenv::dotenv;
use kube::{api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt}, runtime::wait::{await_condition, conditions::is_pod_running}, Client, client, config, Error};
use kube::api::{LogParams, ObjectList};
use kube::config::{Kubeconfig, KubeconfigError, KubeConfigOptions};
use log::{error, info};
use std::{env, process};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Lines};
use std::mem::transmute;
use std::os::unix::raw::mode_t;
use axum::Json;
use axum::service::post;
use crate::api::kube::entity::PodReq;
use crate::kube_cli::get_kube_cli;
use futures::{TryStreamExt, AsyncBufReadExt};
use futures_util::future::err;
use futures_util::{AsyncBufRead, TryFutureExt};
use kube::runtime::{watcher, WatchStreamExt};
use kube::runtime::watcher::{Config, Event};
use tokio::pin;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use crate::api::kube::log_dao::log_entity;

pub(crate) mod entity {
    include!("../entity/kube_req.rs");
}

pub(crate) mod log_dao {
    include!("../dao/log_db.rs");
}

static mut WATCHER_START: Lazy<RwLock<HashMap<String, bool>>> = Lazy::new(|| RwLock::from(HashMap::default()));

static mut POD_LOG: Lazy<RwLock<HashMap<String, bool>>> = Lazy::new(|| RwLock::from(HashMap::default()));

pub async fn run_watcher(namespace: String) {
    tokio::spawn(async move {
        println!("run watch");
        unsafe {
            let start = WATCHER_START.read().await;
            // No need to run watcher again if it is started before.
            if *start.get(namespace.as_str()).unwrap_or(&false) {
                println!("already start,no need to run");
                return;
            }
        }


        let client = get_kube_cli();

        let pods: Api<Pod> = if let Some(c) = client {
            let _cli = c.clone();
            Api::namespaced(_cli, &namespace)
        } else {
            panic!("kube client error")
        };

        // create an event watcher
        let mut watcher = watcher(pods, Config::default());
        pin!(watcher);

        let _namespace = namespace.clone();

        unsafe {
            let mut start = WATCHER_START.write().await;
            start.insert(namespace, true);
        }
        // loop deal the events
        loop {
            println!("loop watch");
            match watcher.try_next().await {
                Ok(event) => {
                    match event {
                        Some(Event::Applied(pod)) => {
                            println!("Pod Applied: {:?}", pod.metadata.name);
                            log_stream(_namespace.clone(), pod.metadata.name.unwrap());
                        }
                        Some(Event::Deleted(pod)) => {
                            println!("Pod Deleted: {:?}", pod.metadata.name);
                        }

                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
    });
}


// get namespaces
pub async fn namespaces() -> ObjectList<Namespace> {
    let client = get_kube_cli();

    let namespaces: Api<Namespace> = if let Some(c) = client {
        let _cli = c.clone();
        Api::all(_cli)
    } else {
        panic!("kube client error")
    };

    let lp = ListParams::default();
    let result = namespaces.list(&lp).await.unwrap();

    return result;
}


// get pods
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


// Create pod.
// For now it's create pod of resnet.
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


// Get logs of specific pod.It return logs once.
pub async fn pod_logs(req: PodReq) -> Result<Vec<String>, Error> {
    let client = get_kube_cli();

    // Manage pods
    // let pods: Api<Pod> = Api::default_namespaced(*client);

    let pods: Api<Pod> = if let Some(c) = client {
        let _cli = c.clone();
        Api::namespaced(_cli, &req.namespace.unwrap_or(String::from("default")))
    } else {
        panic!("kube client error")
    };

    let result = pods.logs(&req.pod_name.unwrap(), &Default::default()).await;

    return match result {
        Ok(logs) => {
            let lines = logs.split("\n").map(|s| s.to_string())
                .filter(|s| !s.is_empty()).collect();
            Ok(lines)
        }

        Err(err) => {
            Err(err)
        }
    };
}


// Get information of specific pod.
pub async fn pod_info(req: PodReq) -> Result<Pod, Error> {
    let client = get_kube_cli();

    // Manage pods
    // let pods: Api<Pod> = Api::default_namespaced(*client);

    let pods: Api<Pod> = if let Some(c) = client {
        let _cli = c.clone();
        Api::namespaced(_cli, &req.namespace.unwrap_or(String::from("default")))
    } else {
        panic!("kube client error")
    };

    let result = pods.get(&req.pod_name.unwrap()).await;
    return match result {
        Ok(pod) => {
            Ok(pod)
        }
        Err(err) => {
            Err(err)
        }
    };
}


// record logs when server started.
pub async fn monitor() {
    tokio::spawn(async move {
        let namespaces = namespaces().await;
        for namespace in namespaces.iter() {
            run_watcher(namespace.metadata.name.clone().unwrap()).await;
            let pods = pod_list(namespace.metadata.name.clone().unwrap()).await;
            for pod in pods.iter() {
                log_stream(namespace.metadata.name.clone().unwrap(), pod.metadata.name.clone().unwrap()).await;
            }
        }
    });
}

// record logs by stream
pub async fn log_stream(namespace: String, pod_name: String) {
    unsafe {
        let logged = POD_LOG.read().await;
        // No need to run watcher again if it is started before.
        if *logged.get(pod_name.as_str()).unwrap_or(&false) {
            println!("already logged,no need to log");
            return;
        }
    }

    tokio::spawn(async move {
        let cli = get_kube_cli();
        let pods: Api<Pod> = if let Some(c) = cli {
            let _cli = c.clone();
            Api::namespaced(_cli, &*namespace)
        } else {
            println!("kube client error");
            return;
        };


        let result = pods.get(&*pod_name).await;
        match result {
            Ok(pod) => {
                // Get current list of logs
                let lp = LogParams {
                    follow: true,
                    ..LogParams::default()
                };

                let result = pods.log_stream(&*pod_name, &lp).await;
                match result {
                    Ok(stream) => {
                        let _pod_name = pod_name.clone();
                        unsafe {
                            let mut logged = POD_LOG.write().await;
                            // No need to run watcher again if it is started before.
                            logged.insert(pod_name, true);
                        }
                        println!("start log stream with pod {:?}", _pod_name.clone());


                        let mut logs_stream = stream.lines();
                        // individual logs may or may not buffer
                        while let line = logs_stream.try_next().await.unwrap() {
                            let _line = String::from(line.unwrap_or("".to_string()));
                            let mut hasher = DefaultHasher::new();
                            _line.hash(&mut hasher);
                            let hash_value = hasher.finish();
                            let lables = pod.metadata.labels.clone();
                            match lables {
                                None => {
                                    return;
                                }
                                Some(_lables) => {
                                    let app = _lables.get("app");
                                    match app {
                                        None => {
                                            return;
                                        }
                                        Some(_app) => {
                                            let log = log_entity::Log {
                                                id: 0,
                                                hash_code: hash_value,
                                                app: _app.to_string(),
                                                pod: _pod_name.clone(),
                                                content: _line.to_string(),
                                                create_time: Default::default(),
                                            };
                                            log_dao::insert_log(log).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("get pod {:?} logs stream failed:{:?}", pod_name, err.to_string());
                        return;
                    }
                }
            }
            Err(err) => {
                println!("get pod {:?} failed:{:?}", pod_name, err.to_string());
                return;
            }
        }
    });
}

